use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Read}; // shorthand for using both use std::io and use std::io::Read
use std::ffi;

#[derive(Debug)]
pub enum Error
{
	Io(io::Error),
	FileContainsNil,
	FailedToGetExePath,
}

impl From<io::Error> for Error
{
	fn from(other: io::Error) -> Self
	{
		Error::Io(other)
	}
}

pub struct Resources
{
	root_path: PathBuf,
}

impl Resources
{
	pub fn from_relative_exe_path(rel_path: &Path) -> Result<Resources, Error>
	{
		let exe_file_name = std::env::current_exe()
			.map_err(|_| Error::FailedToGetExePath)?;
		let exe_path = exe_file_name.parent()
			.ok_or(Error::FailedToGetExePath)?;

		Ok(Resources {
			root_path: exe_path.join(rel_path)
		})
	}

	pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error>
	{
		let mut file = fs::File::open(
			resource_name_to_path(&self.root_path, resource_name)
		)?;

		// allocate buffer of the same size as file
		let mut buffer: Vec<u8> = Vec::with_capacity(
			file.metadata()?.len() as usize + 1
		);
		file.read_to_end(&mut buffer)?;

		// Check for null byte (i is a double reference, so needs to be dereferenced twice)
		if buffer.iter().find(|i| **i == 0).is_some()
		{
			return Err(Error::FileContainsNil);
		}

		Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
	}
}

// Helper function to make unix-style paths work regardless of OS (using '/' as seperator)
fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf
{
	let mut path: PathBuf = root_dir.into();

	for part in location.split("/")
	{
		path = path.join(part);
	}

	path
}