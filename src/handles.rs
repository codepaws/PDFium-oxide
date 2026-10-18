use crate::runtime::{defer_cleanup, loader::LoadedLibrary};
use std::{any::Any, sync::Arc};

fn transfer_finalizer<H: Copy + 'static, C: 'static, R: 'static>(
    handle: H,
    close: fn(&C, H),
    retained: Vec<R>,
    schedule: impl FnOnce(Box<dyn FnOnce(&C) + 'static>),
) {
    schedule(Box::new(move |context| {
        close(context, handle);
        drop(retained);
    }));
}

/// Owns one native handle; parents and external buffers live through its close.
pub(crate) struct ManagedResource<H: Copy + 'static> {
    handle: Option<H>,
    close: fn(&LoadedLibrary, H),
    keep_alive: Vec<Arc<dyn Any + Send + Sync>>,
}
impl<H: Copy + 'static> ManagedResource<H> {
    pub(crate) fn new(
        handle: H,
        close: fn(&LoadedLibrary, H),
        keep_alive: Vec<Arc<dyn Any + Send + Sync>>,
    ) -> Self {
        Self {
            handle: Some(handle),
            close,
            keep_alive,
        }
    }
    pub(crate) fn handle(&self) -> H {
        self.handle.expect("live managed resource")
    }
}
impl<H: Copy + 'static> Drop for ManagedResource<H> {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            let retained = std::mem::take(&mut self.keep_alive);
            transfer_finalizer(handle, self.close, retained, |action| {
                defer_cleanup(move |ctx| action(ctx))
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::transfer_finalizer;
    use std::{
        any::Any,
        cell::RefCell,
        collections::VecDeque,
        rc::Rc,
        sync::{Arc, Mutex},
    };
    struct FakeContext {
        closed: Arc<Mutex<Vec<usize>>>,
    }
    type Action = Box<dyn FnOnce(&FakeContext)>;
    type Queue = Rc<RefCell<VecDeque<Action>>>;
    struct FakeResource {
        handle: Option<usize>,
        keep: Vec<Arc<dyn Any>>,
        schedule: Box<dyn Fn(Action)>,
    }
    fn fake_close(ctx: &FakeContext, handle: usize) {
        ctx.closed.lock().unwrap().push(handle);
    }
    impl Drop for FakeResource {
        fn drop(&mut self) {
            if let Some(handle) = self.handle.take() {
                let keep = std::mem::take(&mut self.keep);
                transfer_finalizer(handle, fake_close, keep, |action| (self.schedule)(action));
            }
        }
    }
    fn fake(queue: &Queue, handle: usize, keep: Vec<Arc<dyn Any>>) -> FakeResource {
        let queue = queue.clone();
        FakeResource {
            handle: Some(handle),
            keep,
            schedule: Box::new(move |action| queue.borrow_mut().push_back(action)),
        }
    }
    fn drain(queue: &Queue, ctx: &FakeContext) {
        loop {
            let action = queue.borrow_mut().pop_front();
            let Some(action) = action else { break };
            action(ctx);
        }
    }
    #[test]
    fn cloned_owner_transfers_one_finalizer() {
        let queue = Rc::new(RefCell::new(VecDeque::new()));
        let closed = Arc::new(Mutex::new(Vec::new()));
        let ctx = FakeContext {
            closed: closed.clone(),
        };
        let owner = Arc::new(fake(&queue, 7, vec![]));
        let clone = owner.clone();
        drop(owner);
        assert!(queue.borrow().is_empty());
        drop(clone);
        assert_eq!(queue.borrow().len(), 1);
        drain(&queue, &ctx);
        assert_eq!(*closed.lock().unwrap(), [7]);
    }
    #[test]
    fn child_owner_retains_parent_until_after_child_close() {
        let queue = Rc::new(RefCell::new(VecDeque::new()));
        let closed = Arc::new(Mutex::new(Vec::new()));
        let ctx = FakeContext {
            closed: closed.clone(),
        };
        let parent = Arc::new(fake(&queue, 2, vec![]));
        let retain: Arc<dyn Any> = parent.clone();
        let child = Arc::new(fake(&queue, 1, vec![retain]));
        drop(parent);
        drop(child);
        drain(&queue, &ctx);
        assert_eq!(*closed.lock().unwrap(), [1, 2]);
    }

    #[test]
    fn retained_values_drop_after_the_close_action_runs() {
        struct Retained(Arc<Mutex<Vec<&'static str>>>);
        impl Drop for Retained {
            fn drop(&mut self) {
                self.0.lock().unwrap().push("retained-drop");
            }
        }

        let queue = Rc::new(RefCell::new(VecDeque::new()));
        let events = Arc::new(Mutex::new(Vec::new()));
        let ctx = FakeContext {
            closed: Arc::new(Mutex::new(Vec::new())),
        };
        let retained: Arc<dyn Any> = Arc::new(Retained(events.clone()));
        let resource = fake(&queue, 9, vec![retained]);
        drop(resource);
        assert!(events.lock().unwrap().is_empty());
        drain(&queue, &ctx);
        assert_eq!(*events.lock().unwrap(), ["retained-drop"]);
    }
}
