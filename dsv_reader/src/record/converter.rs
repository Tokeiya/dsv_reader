use std::error::Error;

pub trait MutConverter {
	type T;
	type E: Error;
	
	fn mut_convert(&mut self, source: &str) -> Result<Self::T, Self::E>;
}

pub trait InstanceConverter: MutConverter {
	fn instance_convert(&self, source: &str) -> Result<Self::T, Self::E>;
}

impl<C: InstanceConverter> MutConverter for C {
	type T = <C as MutConverter>::T; // 型の曖昧さを避けるための書き方
	type E = <C as MutConverter>::E;
	
	fn mut_convert(&mut self, source: &str) -> Result<Self::T, Self::E> {
		// selfを不変参照として扱い、InstanceConverterのメソッドを呼ぶ
		self.instance_convert(source)
	}
}

pub trait StaticConverter: InstanceConverter {
	fn static_convert(source: &str) -> Result<Self::T, Self::E>;
}

impl<S: StaticConverter> InstanceConverter for S {
	fn instance_convert(&self, source: &str) -> Result<Self::T, Self::E> {
		// インスタンス(self)を無視してスタティックメソッドを呼ぶ
		Self::static_convert(source)
	}
}
