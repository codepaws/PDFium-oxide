//! Process-wide PDFium loading and call coordination.
use std::collections::{HashSet, VecDeque};
use std::{
    cell::{Cell, RefCell},
    path::PathBuf,
    sync::{Mutex, OnceLock},
};

use crate::{Capabilities, PdfiumConfig, PdfiumError};

pub(crate) mod loader;

thread_local! { static IN_PDFIUM: Cell<bool> = const { Cell::new(false) }; }
thread_local! { static DEFERRED_CLEANUP: CleanupQueue<loader::LoadedLibrary> = const { CleanupQueue::new() }; }
static PROCESS: OnceLock<Mutex<ProcessState>> = OnceLock::new();

type CleanupAction<T> = Box<dyn FnOnce(&T) + 'static>;

struct CleanupQueue<T> {
    actions: RefCell<VecDeque<CleanupAction<T>>>,
}

impl<T> CleanupQueue<T> {
    const fn new() -> Self {
        Self {
            actions: RefCell::new(VecDeque::new()),
        }
    }

    fn enqueue(&self, action: CleanupAction<T>) {
        self.actions.borrow_mut().push_back(action);
    }

    fn drain(&self, context: &T) {
        loop {
            let action = self.actions.borrow_mut().pop_front();
            let Some(action) = action else {
                break;
            };
            action(context);
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Phase {
    Empty,
    Prepared,
    Initialized,
    Destroyed,
}

struct ProcessState {
    phase: Phase,
    path: Option<PathBuf>,
    config: Option<PdfiumConfig>,
    context: Option<loader::LoadedLibrary>,
    capabilities: Capabilities,
    available_symbols: HashSet<String>,
}
impl Default for ProcessState {
    fn default() -> Self {
        Self {
            phase: Phase::Empty,
            path: None,
            config: None,
            context: None,
            capabilities: Capabilities::default(),
            available_symbols: HashSet::new(),
        }
    }
}
fn process() -> &'static Mutex<ProcessState> {
    PROCESS.get_or_init(|| Mutex::new(ProcessState::default()))
}

fn lock() -> std::sync::MutexGuard<'static, ProcessState> {
    recover_lock(process())
}
fn recover_lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}
fn under_gate<T, R>(gate: &Mutex<T>, operation: impl FnOnce(&T) -> R) -> R {
    let state = recover_lock(gate);
    operation(&state)
}
pub(crate) fn check_reentry() -> Result<(), PdfiumError> {
    if IN_PDFIUM.with(Cell::get) {
        Err(PdfiumError::ReentrantCall)
    } else {
        Ok(())
    }
}

#[cfg(test)]
pub(crate) fn with_reentry_for_test<T>(operation: impl FnOnce() -> T) -> T {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            IN_PDFIUM.with(|value| value.set(false));
        }
    }
    IN_PDFIUM.with(|value| value.set(true));
    let reset = Reset;
    let result = operation();
    drop(reset);
    result
}

fn prepare_with<T>(
    phase: &mut Phase,
    saved_path: &mut Option<PathBuf>,
    saved_config: &mut Option<PdfiumConfig>,
    path: PathBuf,
    config: PdfiumConfig,
    load: impl FnOnce() -> Result<T, PdfiumError>,
) -> Result<Option<T>, PdfiumError> {
    if *phase != Phase::Empty {
        if saved_path.as_ref() == Some(&path)
            && saved_config.as_ref() == Some(&config)
            && *phase != Phase::Destroyed
        {
            return Ok(None);
        }
        return Err(PdfiumError::AlreadyInitialized { path });
    }
    let loaded = load()?;
    *saved_path = Some(path);
    *saved_config = Some(config);
    *phase = Phase::Prepared;
    Ok(Some(loaded))
}

fn require_phase(
    current: Phase,
    expected: Phase,
    function: &'static str,
) -> Result<(), PdfiumError> {
    if current == expected {
        return Ok(());
    }
    if current == Phase::Destroyed {
        return Err(PdfiumError::LibraryDestroyed);
    }
    let phase = match current {
        Phase::Empty => "empty",
        Phase::Prepared => "prepared",
        Phase::Initialized => "initialized",
        Phase::Destroyed => "destroyed",
    };
    Err(PdfiumError::LifecycleViolation {
        function: function.into(),
        phase,
    })
}
struct Entry;
impl Entry {
    fn enter() -> Result<Self, PdfiumError> {
        check_reentry()?;
        IN_PDFIUM.with(|v| v.set(true));
        Ok(Self)
    }
}
impl Drop for Entry {
    fn drop(&mut self) {
        IN_PDFIUM.with(|v| v.set(false));
    }
}

/// Native call return value and error snapshot captured under the coordinator lock.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PdfiumCall<T> {
    /// Return value from the PDFium function.
    pub return_value: T,
    /// Numeric `FPDF_GetLastError` snapshot.
    pub last_error: u64,
}

/// Process-owned initialized PDFium instance.
pub struct Pdfium {
    _private: (),
    capabilities: Capabilities,
}
/// A loaded, symbol-validated library before initialization.
pub struct PdfiumInit {
    _private: (),
    capabilities: Capabilities,
}
/// A terminally destroyed PDFium process context.
pub struct PdfiumDestroyed {
    _private: (),
    capabilities: Capabilities,
}

impl Pdfium {
    /// Loads and initializes the explicitly configured PDFium library.
    pub fn load(config: PdfiumConfig) -> Result<Self, PdfiumError> {
        Self::prepare(config)?.initialize()
    }
    /// Loads and validates exports without initializing PDFium.
    pub fn prepare(mut config: PdfiumConfig) -> Result<PdfiumInit, PdfiumError> {
        check_reentry()?;
        let path =
            std::fs::canonicalize(config.library_path()).map_err(|e| PdfiumError::LibraryLoad {
                path: config.library_path().to_owned(),
                message: e.to_string(),
            })?;
        config.set_canonical_path(path.clone());
        let mut state = lock();
        let ProcessState {
            phase,
            path: saved_path,
            config: saved_config,
            context,
            capabilities,
            available_symbols,
        } = &mut *state;
        let loaded = prepare_with(
            phase,
            saved_path,
            saved_config,
            path.clone(),
            config.clone(),
            || loader::load(&path, &config),
        )?;
        if let Some((loaded_context, loaded_capabilities, loaded_symbols)) = loaded {
            *context = Some(loaded_context);
            *capabilities = loaded_capabilities;
            *available_symbols = loaded_symbols;
        }
        let capabilities = *capabilities;
        Ok(PdfiumInit {
            _private: (),
            capabilities,
        })
    }
    /// Returns whether an optional or declared capability is available.
    pub fn capabilities(&self) -> Capabilities {
        self.capabilities
    }
    /// Returns the initialized raw facade.
    pub fn raw(&self) -> crate::raw::RawApi<'_> {
        crate::raw::RawApi { _owner: self }
    }
    /// Destroys the process-wide library.
    ///
    /// # Safety
    /// The caller must ensure every native resource and retained pointer from every `Pdfium` handle has been released and that no initialized PDFium operation will run afterward. Only the explicitly permitted post-destroy helper may be called after this operation.
    pub unsafe fn destroy_library(self) -> Result<PdfiumDestroyed, PdfiumError> {
        check_reentry()?;
        let _entry = Entry::enter()?;
        let mut state = lock();
        if state.phase != Phase::Initialized {
            return Err(PdfiumError::LifecycleViolation {
                function: "FPDF_DestroyLibrary".into(),
                phase: "not initialized",
            });
        }
        let context = state.context.as_ref().expect("initialized context");
        unsafe {
            context.dispatch.FPDF_DestroyLibrary();
        }
        state.phase = Phase::Destroyed;
        Ok(PdfiumDestroyed {
            _private: (),
            capabilities: self.capabilities,
        })
    }
}
impl PdfiumInit {
    /// Returns the capabilities established while validating the library exports.
    pub fn capabilities(&self) -> Capabilities {
        self.capabilities
    }
    /// Returns the pre-initialization raw facade.
    pub fn raw(&self) -> crate::raw::PreInitApi<'_> {
        crate::raw::PreInitApi { _owner: self }
    }
    /// Initializes the already validated library.
    pub fn initialize(self) -> Result<Pdfium, PdfiumError> {
        check_reentry()?;
        let _entry = Entry::enter()?;
        let mut state = lock();
        let config = state.config.as_ref().cloned();
        let ProcessState { phase, context, .. } = &mut *state;
        let result = initialize_once(phase, || {
            let native = config.expect("prepared config").native_config();
            let context = context.as_ref().expect("prepared context");
            unsafe {
                context.dispatch.FPDF_InitLibraryWithConfig(&native);
            }
        });
        result.map(|_| Pdfium {
            _private: (),
            capabilities: self.capabilities,
        })
    }
}
impl PdfiumDestroyed {
    /// Returns the immutable capabilities recorded before destruction.
    pub fn capabilities(&self) -> Capabilities {
        self.capabilities
    }
    /// Returns the small post-destroy raw facade.
    pub fn raw(&self) -> crate::raw::DestroyedApi<'_> {
        crate::raw::DestroyedApi { _owner: self }
    }
}

fn initialize_once(phase: &mut Phase, initialize: impl FnOnce()) -> Result<bool, PdfiumError> {
    match phase {
        Phase::Prepared => {
            *phase = Phase::Initialized;
            initialize();
            Ok(true)
        }
        Phase::Initialized => Ok(false),
        Phase::Destroyed => Err(PdfiumError::LibraryDestroyed),
        Phase::Empty => Err(PdfiumError::Initialization(
            "library was not prepared".into(),
        )),
    }
}

pub(crate) fn call<T>(
    function: &'static str,
    expected: Phase,
    f: impl FnOnce(&loader::LoadedLibrary) -> T,
) -> Result<PdfiumCall<T>, PdfiumError> {
    call_with_source(function, expected, f, None).map(|(call, _)| call)
}

/// Runs a native operation and observes callbacks from that operation atomically.
pub(crate) fn call_with_source<T>(
    function: &'static str,
    expected: Phase,
    f: impl FnOnce(&loader::LoadedLibrary) -> T,
    source: Option<&dyn crate::callback::SourceFailureCheck>,
) -> Result<(PdfiumCall<T>, Option<PdfiumError>), PdfiumError> {
    check_reentry()?;
    let _entry = Entry::enter()?;
    under_gate(process(), |state| {
        require_phase(state.phase, expected, function)?;
        let context = state
            .context
            .as_ref()
            .ok_or_else(|| PdfiumError::Initialization("library is not loaded".into()))?;
        if !state.available_symbols.contains(function) {
            let capability = crate::raw::symbols::OPTIONAL_SYMBOLS
                .iter()
                .find(|(name, _)| *name == function)
                .map(|(_, capability)| *capability);
            if let Some(capability) = capability {
                return Err(PdfiumError::UnsupportedCapability(capability));
            }
            return Err(PdfiumError::MissingSymbol {
                path: state.path.clone().unwrap_or_default(),
                symbol: function.to_owned(),
            });
        }
        let (result, source_error) = invoke_with_source_operation(
            || f(context),
            || unsafe { context.dispatch.FPDF_GetLastError() as u64 },
            source,
            function,
            || drain_deferred_cleanup(context),
        );
        Ok((result, source_error))
    })
}

/// Runs several related native calls while retaining the process gate for the
/// full sequence. Each call still snapshots PDFium's last error immediately
/// and checks source callback failures independently.
pub(crate) fn call_sequence<T>(
    expected: Phase,
    source: Option<&dyn crate::callback::SourceFailureCheck>,
    operation: impl FnOnce(&mut NativeCallSequence<'_>) -> Result<T, PdfiumError>,
) -> Result<T, PdfiumError> {
    check_reentry()?;
    let _entry = Entry::enter()?;
    under_gate(process(), |state| {
        require_phase(state.phase, expected, "PDFium call sequence")?;
        let context = state
            .context
            .as_ref()
            .ok_or_else(|| PdfiumError::Initialization("library is not loaded".into()))?;
        let _cleanup = DrainDeferredCleanup(context);
        let mut calls = NativeCallSequence {
            context,
            path: state.path.as_ref().expect("loaded library path"),
            available_symbols: &state.available_symbols,
            source,
        };
        operation(&mut calls)
    })
}

struct DrainDeferredCleanup<'a>(&'a loader::LoadedLibrary);

impl Drop for DrainDeferredCleanup<'_> {
    fn drop(&mut self) {
        drain_deferred_cleanup(self.0);
    }
}

pub(crate) struct NativeCallSequence<'a> {
    context: &'a loader::LoadedLibrary,
    path: &'a PathBuf,
    available_symbols: &'a HashSet<String>,
    source: Option<&'a dyn crate::callback::SourceFailureCheck>,
}

impl NativeCallSequence<'_> {
    pub(crate) fn call<T>(
        &mut self,
        function: &'static str,
        operation: impl FnOnce(&loader::LoadedLibrary) -> T,
    ) -> Result<(PdfiumCall<T>, Option<PdfiumError>), PdfiumError> {
        if !self.available_symbols.contains(function) {
            let capability = crate::raw::symbols::OPTIONAL_SYMBOLS
                .iter()
                .find(|(name, _)| *name == function)
                .map(|(_, capability)| *capability);
            if let Some(capability) = capability {
                return Err(PdfiumError::UnsupportedCapability(capability));
            }
            return Err(PdfiumError::MissingSymbol {
                path: self.path.clone(),
                symbol: function.to_owned(),
            });
        }
        Ok(invoke_with_source_operation(
            || operation(self.context),
            || unsafe { self.context.dispatch.FPDF_GetLastError() as u64 },
            self.source,
            function,
            || drain_deferred_cleanup(self.context),
        ))
    }
}

fn invoke_with_snapshot<T>(
    call: impl FnOnce() -> T,
    snapshot: impl FnOnce() -> u64,
) -> PdfiumCall<T> {
    let return_value = call();
    let last_error = snapshot();
    PdfiumCall {
        return_value,
        last_error,
    }
}

#[cfg(test)]
fn invoke_with_snapshot_then<T>(
    call: impl FnOnce() -> T,
    snapshot: impl FnOnce() -> u64,
    after_snapshot: impl FnOnce(),
) -> PdfiumCall<T> {
    let outcome = invoke_with_snapshot(call, snapshot);
    after_snapshot();
    outcome
}

fn invoke_with_source_operation<T>(
    call: impl FnOnce() -> T,
    snapshot: impl FnOnce() -> u64,
    source: Option<&dyn crate::callback::SourceFailureCheck>,
    function: &'static str,
    cleanup: impl FnOnce(),
) -> (PdfiumCall<T>, Option<PdfiumError>) {
    let token = source.map(|source| source.begin_source_operation());
    let outcome = invoke_with_snapshot(call, snapshot);
    let check = source
        .zip(token)
        .and_then(|(source, token)| source.source_failure_since(token, function));
    cleanup();
    (outcome, check)
}

/// Converts an atomic source check into the safe operation result, disposing
/// any native value returned alongside a callback failure.
pub(crate) fn source_checked_result<T>(
    call: PdfiumCall<T>,
    source_error: Option<PdfiumError>,
    dispose: impl FnOnce(T),
) -> Result<PdfiumCall<T>, PdfiumError> {
    if let Some(error) = source_error {
        dispose(call.return_value);
        Err(error)
    } else {
        Ok(call)
    }
}

/// Schedules a native resource close, deferring it when called during PDFium.
///
/// Cleanup closures retain any parent owners and external buffers they need.
/// When run immediately, or later during a call's epilogue, they execute while
/// the process gate is held. Cleanup does not overwrite the triggering call's
/// captured result or error snapshot.
pub(crate) fn defer_cleanup(action: impl FnOnce(&loader::LoadedLibrary) + 'static) {
    if IN_PDFIUM.with(Cell::get) {
        DEFERRED_CLEANUP.with(|queue| queue.enqueue(Box::new(action)));
        return;
    }

    let Ok(_entry) = Entry::enter() else {
        return;
    };
    let state = lock();
    if state.phase != Phase::Initialized {
        return;
    }
    let Some(context) = state.context.as_ref() else {
        return;
    };
    action(context);
    drain_deferred_cleanup(context);
}

fn drain_deferred_cleanup(context: &loader::LoadedLibrary) {
    DEFERRED_CLEANUP.with(|queue| queue.drain(context));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_dispatch_rejects_calls_before_initialization() {
        let error = call("FPDF_LoadDocument", Phase::Initialized, |_| ()).unwrap_err();
        assert!(matches!(
            error,
            PdfiumError::LifecycleViolation { phase: "empty", .. }
        ));
    }

    #[test]
    fn failed_prepare_is_retryable_and_conflicting_configuration_is_rejected() {
        let path = std::env::temp_dir().join("pdfium-runtime-test.dll");
        let config = PdfiumConfig::new(path.clone()).unwrap();
        let mut phase = Phase::Empty;
        let mut saved_path = None;
        let mut saved_config = None;
        let failed: Result<Option<u8>, _> = prepare_with(
            &mut phase,
            &mut saved_path,
            &mut saved_config,
            path.clone(),
            config.clone(),
            || Err(PdfiumError::Initialization("injected load failure".into())),
        );
        assert!(failed.is_err());
        assert_eq!(phase, Phase::Empty);
        assert!(saved_path.is_none() && saved_config.is_none());
        let loaded = prepare_with(
            &mut phase,
            &mut saved_path,
            &mut saved_config,
            path.clone(),
            config.clone(),
            || Ok(3_u8),
        )
        .unwrap();
        assert_eq!(loaded, Some(3));
        assert_eq!(phase, Phase::Prepared);
        let reused: Result<Option<u8>, _> = prepare_with(
            &mut phase,
            &mut saved_path,
            &mut saved_config,
            path.clone(),
            config,
            || panic!("must reuse"),
        );
        assert_eq!(reused.unwrap(), None);
        let other = PdfiumConfig::new(path.with_file_name("other-pdfium.dll")).unwrap();
        assert!(matches!(
            prepare_with(
                &mut phase,
                &mut saved_path,
                &mut saved_config,
                other.library_path().to_owned(),
                other,
                || Ok(4_u8)
            ),
            Err(PdfiumError::AlreadyInitialized { .. })
        ));
    }

    #[test]
    fn lifecycle_phase_checks_cover_stale_and_post_destroyed_facades() {
        assert!(matches!(
            require_phase(Phase::Initialized, Phase::Prepared, "preinit"),
            Err(PdfiumError::LifecycleViolation { .. })
        ));
        assert!(matches!(
            require_phase(Phase::Destroyed, Phase::Initialized, "ordinary"),
            Err(PdfiumError::LibraryDestroyed)
        ));
        assert!(require_phase(
            Phase::Destroyed,
            Phase::Destroyed,
            "FPDF_GetArrayBufferAllocatorSharedInstance"
        )
        .is_ok());
    }

    #[test]
    fn reentry_is_rejected_before_a_call_can_wait_on_the_gate() {
        IN_PDFIUM.with(|value| value.set(true));
        let error = call("FPDF_LoadDocument", Phase::Initialized, |_| ()).unwrap_err();
        IN_PDFIUM.with(|value| value.set(false));
        assert!(matches!(error, PdfiumError::ReentrantCall));
    }

    #[test]
    fn prepare_reentry_is_rejected_before_filesystem_or_loader_work() {
        let config =
            PdfiumConfig::new(std::env::temp_dir().join("not-present-pdfium.dll")).unwrap();
        IN_PDFIUM.with(|value| value.set(true));
        let error = match Pdfium::prepare(config) {
            Err(error) => error,
            Ok(_) => panic!("re-entry unexpectedly loaded PDFium"),
        };
        IN_PDFIUM.with(|value| value.set(false));
        assert!(matches!(error, PdfiumError::ReentrantCall));
    }

    #[test]
    fn capability_lookup_is_callback_safe_and_does_not_lock_coordinator() {
        let instance = Pdfium {
            _private: (),
            capabilities: Capabilities::new(true, false),
        };
        let gate = process().lock().unwrap();
        let (send, receive) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            send.send(instance.capabilities().supports(crate::Capability::V8Xfa))
                .unwrap();
        });
        assert!(receive
            .recv_timeout(std::time::Duration::from_secs(2))
            .unwrap());
        drop(gate);
    }

    #[test]
    fn panic_clears_the_thread_local_reentry_marker() {
        let result = std::panic::catch_unwind(|| {
            let _entry = Entry::enter().unwrap();
            panic!("simulated PDFium callback panic");
        });
        assert!(result.is_err());
        assert!(!IN_PDFIUM.with(Cell::get));
    }

    #[test]
    fn concurrent_compatible_initialization_invokes_native_initializer_once() {
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            Arc, Mutex,
        };
        let phase = Arc::new(Mutex::new(Phase::Prepared));
        let count = Arc::new(AtomicUsize::new(0));
        let workers: Vec<_> = (0..12)
            .map(|_| {
                let phase = Arc::clone(&phase);
                let count = Arc::clone(&count);
                std::thread::spawn(move || {
                    let mut phase = phase.lock().unwrap();
                    initialize_once(&mut phase, || {
                        count.fetch_add(1, Ordering::SeqCst);
                    })
                    .unwrap();
                })
            })
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn native_error_snapshot_is_taken_immediately_after_the_operation() {
        use std::cell::RefCell;
        let sequence = RefCell::new(Vec::new());
        let outcome = invoke_with_snapshot(
            || {
                sequence.borrow_mut().push("operation");
                17
            },
            || {
                sequence.borrow_mut().push("get_last_error");
                9
            },
        );
        assert_eq!(outcome.return_value, 17);
        assert_eq!(outcome.last_error, 9);
        assert_eq!(*sequence.borrow(), ["operation", "get_last_error"]);
    }

    #[test]
    fn later_safe_operation_returns_callback_error_and_closes_returned_handle() {
        use crate::callback::{SourceContext, SourceFailureCheck};
        use std::{
            io::Cursor,
            sync::{Arc, Mutex},
        };
        let source = SourceContext::new(Cursor::new(b"abc".to_vec()), 3);
        let source_check: Arc<dyn SourceFailureCheck> = source.clone();
        let gate = Mutex::new(());
        let closed = Mutex::new(Vec::new());
        let operation = under_gate(&gate, |_| {
            let callback = source.interface.m_GetBlock.unwrap();
            let mut out = [0u8; 2];
            let (native, failure) = invoke_with_source_operation(
                || {
                    assert_eq!(
                        unsafe { callback(source.interface.m_Param, 2, out.as_mut_ptr(), 2) },
                        0
                    );
                    42usize
                },
                || 7,
                Some(source_check.as_ref()),
                "LaterSafeOperation",
                || {},
            );
            assert_eq!(native.last_error, 7);
            source_checked_result(native, failure, |handle| {
                closed.lock().unwrap().push(handle)
            })
        });
        assert!(
            matches!(operation, Err(PdfiumError::CallbackRead { ref operation, .. }) if operation == "LaterSafeOperation")
        );
        assert_eq!(*closed.lock().unwrap(), [42]);
    }

    #[test]
    fn concurrent_operations_attribute_shared_source_failure_to_triggering_call() {
        use crate::callback::{SourceContext, SourceFailureCheck};
        use std::{
            io::{self, Read, Seek, SeekFrom},
            sync::{mpsc, Arc, Mutex},
        };
        struct WaitForContender(std::sync::mpsc::Receiver<()>, bool);
        impl Read for WaitForContender {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                if !self.1 {
                    self.0.recv().unwrap();
                    self.1 = true;
                }
                Err(io::Error::other("triggering read failure"))
            }
        }
        impl Seek for WaitForContender {
            fn seek(&mut self, _: SeekFrom) -> io::Result<u64> {
                Ok(0)
            }
        }
        let (contender_tx, contender_rx) = mpsc::channel();
        let source = SourceContext::new(WaitForContender(contender_rx, false), 3);
        let source_check: Arc<dyn SourceFailureCheck> = source.clone();
        let gate = Arc::new(Mutex::new(()));
        let (send, receive) = mpsc::channel();
        let first_gate = gate.clone();
        let first_source = source.clone();
        let first_check = source_check.clone();
        let first = std::thread::spawn(move || {
            let result = under_gate(&first_gate, |_| {
                let callback = first_source.interface.m_GetBlock.unwrap();
                let mut out = [0u8; 2];
                let (native, failure) = invoke_with_source_operation(
                    || {
                        assert_eq!(
                            unsafe {
                                callback(first_source.interface.m_Param, 0, out.as_mut_ptr(), 2)
                            },
                            0
                        );
                        1
                    },
                    || 0,
                    Some(first_check.as_ref()),
                    "OperationA",
                    || {},
                );
                source_checked_result(native, failure, |_| {})
            });
            result
        });
        let second_gate = gate.clone();
        let second_check = source_check.clone();
        let second_source = source.clone();
        let second = std::thread::spawn(move || {
            loop {
                match second_gate.try_lock() {
                    Err(std::sync::TryLockError::WouldBlock) => {
                        send.send(()).unwrap();
                        break;
                    }
                    Ok(guard) => {
                        drop(guard);
                        std::thread::yield_now();
                    }
                    Err(std::sync::TryLockError::Poisoned(error)) => {
                        panic!("gate poisoned: {error}")
                    }
                }
            }
            under_gate(&second_gate, |_| {
                let (native, failure) = invoke_with_source_operation(
                    || 2,
                    || 0,
                    Some(second_check.as_ref()),
                    "OperationB",
                    || {},
                );
                let successful = source_checked_result(native, failure, |_| {}).unwrap();
                assert_eq!(
                    successful.return_value, 2,
                    "unrelated operation must not inherit another call's callback error"
                );
                let callback = second_source.interface.m_GetBlock.unwrap();
                let mut byte = 0;
                let (native, failure) = invoke_with_source_operation(
                    || {
                        assert_eq!(
                            unsafe { callback(second_source.interface.m_Param, 0, &mut byte, 1) },
                            0
                        );
                        3
                    },
                    || 0,
                    Some(second_check.as_ref()),
                    "OperationC",
                    || {},
                );
                source_checked_result(native, failure, |_| {})
            })
        });
        receive.recv().unwrap();
        contender_tx.send(()).unwrap();
        assert!(
            matches!(first.join().unwrap(), Err(PdfiumError::CallbackRead { ref operation, .. }) if operation == "OperationA")
        );
        assert!(
            matches!(second.join().unwrap(), Err(PdfiumError::CallbackRead { ref operation, .. }) if operation == "OperationC")
        );
    }

    #[test]
    fn poisoned_coordinator_mutex_is_recovered() {
        let mutex = Mutex::new(0_u8);
        let _ = std::panic::catch_unwind(|| {
            let _guard = mutex.lock().unwrap();
            panic!("poison test mutex");
        });
        *recover_lock(&mutex) = 2;
        assert_eq!(*recover_lock(&mutex), 2);
    }

    #[test]
    fn production_call_gate_serializes_concurrent_operations() {
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        };
        let gate = Arc::new(Mutex::new(()));
        let active = Arc::new(AtomicUsize::new(0));
        let maximum = Arc::new(AtomicUsize::new(0));
        let workers: Vec<_> = (0..10)
            .map(|_| {
                let gate = Arc::clone(&gate);
                let active = Arc::clone(&active);
                let maximum = Arc::clone(&maximum);
                std::thread::spawn(move || {
                    under_gate(&gate, |_| {
                        let now = active.fetch_add(1, Ordering::SeqCst) + 1;
                        maximum.fetch_max(now, Ordering::SeqCst);
                        std::thread::yield_now();
                        active.fetch_sub(1, Ordering::SeqCst);
                    })
                })
            })
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
        assert_eq!(maximum.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn call_sequence_keeps_the_gate_between_related_calls() {
        use std::sync::{mpsc, Arc};

        let gate = Arc::new(Mutex::new(()));
        let (started_tx, started_rx) = mpsc::channel();
        let (blocked_tx, blocked_rx) = mpsc::channel();
        let (finish_tx, finish_rx) = mpsc::channel();
        let events = Arc::new(Mutex::new(Vec::new()));

        let sequence_gate = Arc::clone(&gate);
        let sequence_events = Arc::clone(&events);
        let sequence = std::thread::spawn(move || {
            under_gate(&sequence_gate, |_| {
                sequence_events.lock().unwrap().push("first native call");
                started_tx.send(()).unwrap();
                finish_rx.recv().unwrap();
                sequence_events.lock().unwrap().push("last native call");
            });
        });

        started_rx.recv().unwrap();
        let other_gate = Arc::clone(&gate);
        let other_events = Arc::clone(&events);
        let other = std::thread::spawn(move || {
            loop {
                match other_gate.try_lock() {
                    Err(std::sync::TryLockError::WouldBlock) => {
                        blocked_tx.send(()).unwrap();
                        break;
                    }
                    Ok(guard) => {
                        drop(guard);
                        std::thread::yield_now();
                    }
                    Err(std::sync::TryLockError::Poisoned(error)) => {
                        panic!("gate poisoned: {error}")
                    }
                }
            }
            under_gate(&other_gate, |_| {
                other_events.lock().unwrap().push("concurrent save");
            });
        });

        blocked_rx.recv().unwrap();
        finish_tx.send(()).unwrap();
        sequence.join().unwrap();
        other.join().unwrap();
        assert_eq!(
            *events.lock().unwrap(),
            ["first native call", "last native call", "concurrent save"]
        );
    }

    #[test]
    fn cleanup_queue_releases_borrow_before_running_records_and_parent_drops() {
        use std::cell::RefCell;
        use std::rc::Rc;
        use std::sync::{Arc, Weak};

        struct FakeContext;
        struct ParentOnDrop {
            queue: Rc<CleanupQueue<FakeContext>>,
            events: Rc<RefCell<Vec<&'static str>>>,
        }
        impl Drop for ParentOnDrop {
            fn drop(&mut self) {
                let events = Rc::clone(&self.events);
                self.queue.enqueue(Box::new(move |_| {
                    events.borrow_mut().push("parent-close");
                }));
            }
        }
        struct StorageCheck {
            storage: Weak<()>,
            events: Rc<RefCell<Vec<&'static str>>>,
        }
        impl Drop for StorageCheck {
            fn drop(&mut self) {
                assert!(self.storage.upgrade().is_none());
                self.events
                    .borrow_mut()
                    .push("backing-released-after-close");
            }
        }

        let queue = Rc::new(CleanupQueue::new());
        let events = Rc::new(RefCell::new(Vec::new()));
        let storage = Arc::new(());
        let weak_storage = Arc::downgrade(&storage);
        let parent = ParentOnDrop {
            queue: Rc::clone(&queue),
            events: Rc::clone(&events),
        };
        let events_for_child = Rc::clone(&events);
        queue.enqueue(Box::new(move |_| {
            assert!(Arc::strong_count(&storage) == 1);
            events_for_child.borrow_mut().push("child-close");
            let backing_released = StorageCheck {
                storage: weak_storage,
                events: Rc::clone(&events_for_child),
            };
            drop(storage);
            drop(parent);
            drop(backing_released);
        }));

        queue.drain(&FakeContext);

        assert_eq!(
            *events.borrow(),
            [
                "child-close",
                "backing-released-after-close",
                "parent-close"
            ]
        );
    }

    #[test]
    fn cleanup_runs_after_return_and_error_snapshot_even_when_callback_panics() {
        use std::cell::RefCell;

        let events = RefCell::new(Vec::new());
        let outcome = invoke_with_snapshot_then(
            || {
                events.borrow_mut().push("operation-start");
                let callback = std::panic::catch_unwind(|| panic!("caught callback panic"));
                assert!(callback.is_err());
                events.borrow_mut().push("operation-return");
                17
            },
            || {
                events.borrow_mut().push("last-error-snapshot");
                9
            },
            || events.borrow_mut().push("deferred-close"),
        );

        assert_eq!(outcome.return_value, 17);
        assert_eq!(outcome.last_error, 9);
        assert_eq!(
            *events.borrow(),
            [
                "operation-start",
                "operation-return",
                "last-error-snapshot",
                "deferred-close"
            ]
        );
    }
}
