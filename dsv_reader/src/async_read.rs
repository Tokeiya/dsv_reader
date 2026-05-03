use super::async_token_stream::AsyncTokenStream;
pub trait AsyncDsvRead<R: AsyncTokenStream> {
	fn read(&mut self) -> impl Future<Output = Result<Option<String>, R::Error>> + '_;
	
	fn fill(&mut self, buffer: &'_ mut Vec<String>) -> impl Future<Output = Result<Option<usize>, R::Error>> + '_;
}