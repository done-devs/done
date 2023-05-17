use chrono::NaiveDateTime;
use done_local_storage::models::Task;
use relm4::prelude::DynamicIndex;

use super::model::{DateDay, DateTpe};

#[derive(Debug)]
pub enum TaskDetailsFactoryInput {
	SaveTask,
	SetTitle(String),
	SetNotes(Option<String>),
	SetPriority(i32),
	SetFavorite(bool),
	SetStatus(bool),
	SetToday(bool),
	CreateSubTask,
	AddTag(String),
	RemoveTag(DynamicIndex),
	UpdateSubTask(DynamicIndex, Task),
	RemoveSubTask(DynamicIndex),
	SetDueDate(Option<NaiveDateTime>),
	SetReminderDate(Option<NaiveDateTime>),
	CancelWarning,
	SetDate(DateTpe, DateDay),
}

#[derive(Debug)]
pub enum TaskDetailsFactoryOutput {
	SaveTask(Option<DynamicIndex>, Box<Task>, bool),
	CleanTaskEntry,
	HideFlap,
}
