use crate::status::Status;

// 我们已经在早期的练习中看到了如何声明模块，但
// 我们还没有看到如何将它们提取到单独的文件中。
// 现在就来修复它！
//
// 在最简单的情况下，当提取的模块是单个文件时，只需
// 创建一个与模块同名的文件，并将模块内容移到那里。
// 模块文件应放置在与声明该模块的文件相同的目录中。
// 在这种情况下是 `src/lib.rs`，因此 `status.rs` 应放在 `src` 目录下。
mod status;

// TODO: Add a new error variant to `TicketNewError` for when the status string is invalid.
//   当对该变体的错误调用 `source` 时，应返回 `ParseStatusError` 而不是 `None`。

#[derive(Debug, thiserror::Error)]
pub enum TicketNewError {
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionTooLong,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Self, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

        // TODO: Parse the status string into a `Status` enum.

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use common::{valid_description, valid_title};
    use std::error::Error;

    use super::*;

    #[test]
    fn invalid_status() {
        let err = Ticket::new(valid_title(), valid_description(), "invalid".into()).unwrap_err();
        assert_eq!(
            err.to_string(),
            "`invalid` is not a valid status. Use one of: ToDo, InProgress, Done"
        );
        assert!(err.source().is_some());
    }
}
