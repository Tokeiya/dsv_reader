use super::async_token_stream::AsyncTokenStream;
use super::read_error::Error as ReadError;
pub trait AsyncDsvRead<R: AsyncTokenStream> {
	fn read(&mut self) -> impl Future<Output = Result<Option<(String, bool)>, ReadError<R::Error>>> + '_;
	
	async fn fill<'a>(&'a mut self, buffer: &'a mut Vec<String>) -> impl Future<Output = Result<Option<usize>, ReadError<R::Error>>> + 'a {
		async move {
			let mut cnt = 0usize;
			
			loop {
				if let Some((s, b)) = self.read().await? {
					cnt += 1;
					buffer.push(s);
					
					if b {
						break;
					}
				} else {
					break;
				}
			}
			Ok(Some(cnt))
		}
	}
}