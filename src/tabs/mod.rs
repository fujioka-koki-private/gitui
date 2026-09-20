/*!
The tabs module contains a struct for each of the tabs visible in the
ui:

- [`Status`]: Stage changes, push, pull
- [`Revlog`]: Revision log (think git log)
- [`FilesTab`]: See content of any file at HEAD. Blame

Many of the tabs can expand to show more details. This is done via
Enter or right-arrow. To close again, press ESC.
*/

mod files;
mod revlog;
mod status;

pub use files::FilesTab;
pub use revlog::Revlog;
pub use status::Status;
