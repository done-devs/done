use chrono::{DateTime, Utc};
use core_done::models::{recurrence::Day, task::Task};
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
	SetDueDate(Option<DateTime<Utc>>),
	SetReminderDate(Option<DateTime<Utc>>),
	SetReminderHour(u32),
	SetReminderMinute(u32),
	SetDayInRecurrence((bool, Day)),
	CancelWarning,
	SetDate(DateTpe, DateDay),
}

#[derive(Debug)]
pub enum TaskDetailsFactoryOutput {
	SaveTask(Option<DynamicIndex>, Box<Task>, bool),
	CleanTaskEntry,
	HideFlap,
}