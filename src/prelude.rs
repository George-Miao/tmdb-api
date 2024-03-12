use std::{borrow::Cow, future::Future};

pub trait Command: Sync {
    type Output: serde::de::DeserializeOwned;

    fn path(&self) -> Cow<'static, str>;
    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)>;

    fn execute<'a>(
        &'a self,
        client: &'a crate::Client,
    ) -> impl Future<Output = Result<Self::Output, crate::error::Error>> + Send + Sync + 'a
    where
        Self: Sync,
    {
        async {
            let path = self.path();
            client.execute(path.as_ref(), self.params()).await
        }
    }
}
