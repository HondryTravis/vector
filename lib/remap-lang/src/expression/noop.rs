use crate::{CompilerState, Expression, Object, ResolveKind, Result, State, Value};

#[derive(Debug, Clone)]
pub struct Noop;

impl Expression for Noop {
    fn execute(&self, _: &mut State, _: &mut dyn Object) -> Result<Option<Value>> {
        Ok(None)
    }

    fn resolves_to(&self, _: &CompilerState) -> ResolveKind {
        ResolveKind::Maybe(Box::new(ResolveKind::Any))
    }
}
