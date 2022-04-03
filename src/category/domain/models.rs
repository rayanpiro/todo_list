use crate::shared::category::category_id::CategoryId;

use crate::shared::task::task_id::TaskId;

pub struct CategoryName(String);

pub struct Category {
    id: CategoryId,
    name: CategoryName,
}