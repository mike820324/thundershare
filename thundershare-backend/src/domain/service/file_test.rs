use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::{uuid, Uuid};

use crate::domain::{entity::file_meta::FileMeta, error::file::FileError, repository::file_meta::MockFileMetaRepositoryTrait};

use super::file::{FileServiceImpl, FileServiceTrait, MockFileUploaderTrait};

enum FileSvcTestContextExpectedResult {
    WithFileMetaResult(Result<FileMeta, FileError>),
    WithFileMetaListResult(Result<Vec<FileMeta>, FileError>),
}

struct FileSvcTestContext {
    pub input: FileMeta,
    pub setup_fn: Box<dyn Fn() -> Arc<dyn FileServiceTrait>>,
    pub expected: FileSvcTestContextExpectedResult,
}

impl FileSvcTestContext {
    fn new(
        input: FileMeta,
        setup_fn: impl Fn() -> Arc<dyn FileServiceTrait> + 'static,
        expected: FileSvcTestContextExpectedResult,
    ) -> FileSvcTestContext {
        FileSvcTestContext {
            input,
            setup_fn: Box::new(setup_fn),
            expected,
        }
    }
}

#[actix_rt::test]
async fn test_file_svc_read_by_id() {
    let test_context = vec![
        FileSvcTestContext::new(
            FileMeta::new(""),
            || {
                let mock_file_meta_repo = {
                    let mut mock_repo = MockFileMetaRepositoryTrait::new();

                    mock_repo
                        .expect_get_file_meta_by_id()
                        .times(1)
                        .returning(move |_id| Ok(vec![FileMeta::new("")]));

                    mock_repo
                };

                let mock_file_uploader = {
                    let mut mock_repo = MockFileUploaderTrait::new();

                    mock_repo
                };

                let svc = {
                    let file_uploader = Arc::new(mock_file_uploader);
                    let file_meta_repo = Arc::new(RwLock::new(mock_file_meta_repo));
                    FileServiceImpl::new(file_uploader, file_meta_repo)
                };

                svc
            },
            FileSvcTestContextExpectedResult::WithFileMetaResult(Ok(FileMeta::new(""))),
        ),
        FileSvcTestContext::new(
            FileMeta::new(""),
            || {
                let mock_file_meta_repo = {
                    let mut mock_repo = MockFileMetaRepositoryTrait::new();

                    mock_repo
                        .expect_get_file_meta_by_id()
                        .times(1)
                        .returning(move |_id| Ok(vec![]));

                    mock_repo
                };

                let mock_file_uploader = {
                    let mut mock_repo = MockFileUploaderTrait::new();

                    mock_repo
                };

                let svc = {
                    let file_uploader = Arc::new(mock_file_uploader);
                    let file_meta_repo = Arc::new(RwLock::new(mock_file_meta_repo));
                    FileServiceImpl::new(file_uploader, file_meta_repo)
                };
                svc
            },
            FileSvcTestContextExpectedResult::WithFileMetaResult(Err(FileError::FileNotFound)),
        ),
        FileSvcTestContext::new(
            FileMeta::new(""),
            || {
                let mock_file_meta_repo = {
                    let mut mock_repo = MockFileMetaRepositoryTrait::new();

                    mock_repo
                        .expect_get_file_meta_by_id()
                        .times(1)
                        .returning(move |_id| Ok(vec![FileMeta::new_full(&Uuid::default(), &uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"), "")]));

                    mock_repo
                };

                let mock_file_uploader = {
                    let mut mock_repo = MockFileUploaderTrait::new();

                    mock_repo
                };

                let svc = {
                    let file_uploader = Arc::new(mock_file_uploader);
                    let file_meta_repo = Arc::new(RwLock::new(mock_file_meta_repo));
                    FileServiceImpl::new(file_uploader, file_meta_repo)
                };

                svc
            },
            FileSvcTestContextExpectedResult::WithFileMetaResult(Err(FileError::FileNotBelongToCustomer)),
        ),
    ];

    for t in test_context {
        let svc = (t.setup_fn)();
        let result = svc
            .file_read_by_id(&Uuid::default(), &Uuid::default())
            .await
            .map_err(|err| err.downcast().unwrap());

        let FileSvcTestContextExpectedResult::WithFileMetaResult(expected_result) = t.expected
        else {
            return;
        };
        assert_eq!(result, expected_result);
    }
}

#[actix_rt::test]
async fn test_file_svc_list_by_customer_id() {
    let test_context = vec![
        FileSvcTestContext::new(
            FileMeta::new(""),
            || {
                let mock_file_meta_repo = {
                    let mut mock_repo = MockFileMetaRepositoryTrait::new();

                    mock_repo
                        .expect_list_file_meta_by_customer_id()
                        .times(1)
                        .returning(move |_id| Ok(vec![FileMeta::new("")]));

                    mock_repo
                };

                let mock_file_uploader = {
                    let mut mock_repo = MockFileUploaderTrait::new();

                    mock_repo
                };

                let svc = {
                    let file_uploader = Arc::new(mock_file_uploader);
                    let file_meta_repo = Arc::new(RwLock::new(mock_file_meta_repo));
                    FileServiceImpl::new(file_uploader, file_meta_repo)
                };

                svc
            },
            FileSvcTestContextExpectedResult::WithFileMetaListResult(Ok(vec![FileMeta::new("")])),
        ),
        FileSvcTestContext::new(
            FileMeta::new(""),
            || {
                let mock_file_meta_repo = {
                    let mut mock_repo = MockFileMetaRepositoryTrait::new();

                    mock_repo
                        .expect_list_file_meta_by_customer_id()
                        .times(1)
                        .returning(move |_id| Ok(vec![]));

                    mock_repo
                };

                let mock_file_uploader = {
                    let mut mock_repo = MockFileUploaderTrait::new();

                    mock_repo
                };

                let svc = {
                    let file_uploader = Arc::new(mock_file_uploader);
                    let file_meta_repo = Arc::new(RwLock::new(mock_file_meta_repo));
                    FileServiceImpl::new(file_uploader, file_meta_repo)
                };

                svc
            },
            FileSvcTestContextExpectedResult::WithFileMetaListResult(Ok(vec![])),
        ),
    ];

    for t in test_context {
        let svc = (t.setup_fn)();
        let result = svc
            .file_list_by_customer_id(&Uuid::default())
            .await
            .map_err(|err| err.downcast().unwrap());

        let FileSvcTestContextExpectedResult::WithFileMetaListResult(expected_result) = t.expected
        else {
            return;
        };
        assert_eq!(result, expected_result);
    }
}

#[actix_rt::test]
async fn test_file_svc_file_upload() {
    let test_context = vec![
        FileSvcTestContext::new(
            FileMeta::new(""),
            || {
                let mock_file_meta_repo = {
                    let mut mock_repo = MockFileMetaRepositoryTrait::new();
                    mock_repo.expect_create()
                    .times(1)
                    .returning(|_url| {Ok(FileMeta::new(""))});

                    mock_repo
                };

                let mock_file_uploader = {
                    let mut mock_repo = MockFileUploaderTrait::new();

                    mock_repo.expect_upload()
                    .times(1)
                    .returning(|_src, _dest| {Ok(())});

                    mock_repo
                };

                let svc = {
                    let file_uploader = Arc::new(mock_file_uploader);
                    let file_meta_repo = Arc::new(RwLock::new(mock_file_meta_repo));
                    FileServiceImpl::new(file_uploader, file_meta_repo)
                };

                svc
            },
            FileSvcTestContextExpectedResult::WithFileMetaResult(Ok(FileMeta::new(""))),
        ),
    ];

    for t in test_context {
        let svc = (t.setup_fn)();
        let result = svc
            .file_upload("")
            .await
            .map_err(|err| err.downcast().unwrap());

        let FileSvcTestContextExpectedResult::WithFileMetaResult(expected_result) = t.expected
        else {
            return;
        };
        assert_eq!(result, expected_result);
    }
}