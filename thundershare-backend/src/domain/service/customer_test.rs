use std::sync::Arc;

use chrono::{DateTime, Duration, Utc, TimeZone};
use tokio::sync::RwLock;

use crate::domain::{
    entity::{customer::Customer, identity::Identity}, error::customer::CustomerError,
    repository::{customer::MockCustomerRepositoryTrait, used_token::MockUsedTokenRepositoryTrait},
};

use super::customer::{CustomerServiceImpl, CustomerServiceTrait};

enum CustomerTestContextExpectedResult {
    WithIdentityResult(Result<Identity, CustomerError>),
    WithCustomerResult(Result<Customer, CustomerError>),
    WithoutCustomerResult(Result<(), CustomerError>),
}

struct CustomerSvcTestContext {
    pub input_customer: Customer,
    pub setup_fn: Box<dyn Fn() -> Arc<dyn CustomerServiceTrait>>,
    pub expected: CustomerTestContextExpectedResult,
}

impl CustomerSvcTestContext {
    fn new(
        input_customer: Customer,
        setup_fn: impl Fn() -> Arc<dyn CustomerServiceTrait> + 'static,
        expected: CustomerTestContextExpectedResult,
    ) -> CustomerSvcTestContext {
        CustomerSvcTestContext {
            input_customer,
            setup_fn: Box::new(setup_fn),
            expected,
        }
    }
}

fn fake_issue_at() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(1990, 3, 3, 0, 0, 0).unwrap()
}

#[actix_rt::test]
async fn test_customer_svc_signup() {
    let test_context = vec![
        CustomerSvcTestContext::new(
            Customer::new("mikejiang"),
            || {
                let mock_used_token_repo = MockUsedTokenRepositoryTrait::new();
                let mock_customer_repo = {
                    let mut mock_repo = MockCustomerRepositoryTrait::new();

                    mock_repo
                        .expect_get_customer_by_username()
                        .times(1)
                        .returning(move |_username| Ok(vec![]));

                    mock_repo
                        .expect_create_customer()
                        .times(1)
                        .returning(|username, password| Ok(Customer::new(username)));

                    mock_repo
                };

                let svc = {
                    let customer_repo = Arc::new(RwLock::new(mock_customer_repo));
                    let used_token_repo = Arc::new(RwLock::new(mock_used_token_repo));
                    CustomerServiceImpl::new(fake_issue_at, customer_repo, used_token_repo)
                };

                svc
            },
            CustomerTestContextExpectedResult::WithIdentityResult(Ok(
                Identity::new(
                    &Customer::new("mikejiang"),
                    &fake_issue_at(),
                    Duration::minutes(10),
                )
            )),
        ),
        CustomerSvcTestContext::new(
            Customer::new("brucewayne"),
            || {
                let mock_used_token_repo = MockUsedTokenRepositoryTrait::new();
                let mock_customer_repo = {
                    let mut mock_repo = MockCustomerRepositoryTrait::new();

                    mock_repo
                        .expect_get_customer_by_username()
                        .times(1)
                        .returning(move |_username| Ok(vec![Customer::new("")]));
                    mock_repo
                };

                let svc = {
                    let customer_repo = Arc::new(RwLock::new(mock_customer_repo));
                    let used_token_repo = Arc::new(RwLock::new(mock_used_token_repo));
                    CustomerServiceImpl::new(fake_issue_at, customer_repo, used_token_repo)
                };

                svc
            },
            CustomerTestContextExpectedResult::WithIdentityResult(Err(CustomerError::CustomerAlreadyExist)),
        ),
    ];

    for t in test_context {
        let svc = (t.setup_fn)();
        let result = svc
            .customer_signup(
                &t.input_customer.get_username(),
                "",
            )
            .await
            .map_err(|err| err.downcast().unwrap());

        let CustomerTestContextExpectedResult::WithIdentityResult(expected_result) = t.expected else {return;};
        assert_eq!(result, expected_result);
    }
}

#[actix_rt::test]
async fn test_customer_svc_signin() {
    let test_context = vec![
        CustomerSvcTestContext::new(
            Customer::new("mikejiang"),
            || {
                let mock_used_token_repo = MockUsedTokenRepositoryTrait::new();
                let mock_customer_repo = {
                    let mut mock_repo = MockCustomerRepositoryTrait::new();

                    mock_repo
                        .expect_get_customer_by_credential()
                        .times(1)
                        .returning(move |username, password| {
                            Ok(vec![Customer::new(username)])
                        });

                    mock_repo
                };

                let svc = {
                    let customer_repo = Arc::new(RwLock::new(mock_customer_repo));
                    let used_token_repo = Arc::new(RwLock::new(mock_used_token_repo));
                    CustomerServiceImpl::new(fake_issue_at, customer_repo, used_token_repo)
                };

                svc
            },
            CustomerTestContextExpectedResult::WithIdentityResult(Ok(
                Identity::new(
                    &Customer::new("mikejiang"),
                    &fake_issue_at(),
                    Duration::minutes(10),
                )
            )),
        ),
        CustomerSvcTestContext::new(
            Customer::new("brucewayne"),
            || {
                let mock_used_token_repo = MockUsedTokenRepositoryTrait::new();
                let mock_customer_repo = {
                    let mut mock_repo = MockCustomerRepositoryTrait::new();

                    mock_repo
                        .expect_get_customer_by_credential()
                        .times(1)
                        .returning(move |_username, password| Ok(vec![]));
                    mock_repo
                };

                let svc = {
                    let customer_repo = Arc::new(RwLock::new(mock_customer_repo));
                    let used_token_repo = Arc::new(RwLock::new(mock_used_token_repo));
                    CustomerServiceImpl::new(fake_issue_at, customer_repo, used_token_repo)
                };

                svc
            },
            CustomerTestContextExpectedResult::WithIdentityResult(Err(CustomerError::CustomerInvalidCredential)),
        ),
    ];

    for t in test_context {
        let svc = (t.setup_fn)();
        let result = svc
            .customer_signin(
                &t.input_customer.get_username(),
                "",
            )
            .await
            .map_err(|err| err.downcast().unwrap());

        let CustomerTestContextExpectedResult::WithIdentityResult(expected_result) = t.expected else {return;};
        assert_eq!(result, expected_result);
    }
}

#[actix_rt::test]
async fn test_customer_svc_signout() {
    let test_context = vec![
        CustomerSvcTestContext::new(
            Customer::new("mikejiang"),
            || {
                let mock_used_token_repo = {
                    let mut mock_repo = MockUsedTokenRepositoryTrait::new();

                    mock_repo
                        .expect_create_used_token()
                        .times(1)
                        .returning(move |token, expire_time| {
                            Ok(())
                        });

                    mock_repo
                };

                let mock_customer_repo = MockCustomerRepositoryTrait::new();

                let svc = {
                    let customer_repo = Arc::new(RwLock::new(mock_customer_repo));
                    let used_token_repo = Arc::new(RwLock::new(mock_used_token_repo));
                    CustomerServiceImpl::new(fake_issue_at, customer_repo, used_token_repo)
                };

                svc
            },
            CustomerTestContextExpectedResult::WithoutCustomerResult(Ok(())),
        ),
    ];

    for t in test_context {
        let svc = (t.setup_fn)();
        let duration = Duration::minutes(10);

        let identity = Identity::new(&t.input_customer, &fake_issue_at(), duration);
        let result = svc
            .customer_signout(&identity)
            .await
            .map_err(|err| err.downcast().unwrap());

        let CustomerTestContextExpectedResult::WithoutCustomerResult(expected_result) = t.expected else {return;};
        assert_eq!(result, expected_result);
    }
}

#[actix_rt::test]
async fn test_customer_svc_get_customer_by_username() {
    let test_context = vec![
        CustomerSvcTestContext::new(
            Customer::new("mikejiang"),
            || {
                let mock_used_token_repo = {
                    let mut mock_repo = MockUsedTokenRepositoryTrait::new();
                    mock_repo
                };

                let mock_customer_repo = {
                    let mut mock_repo = MockCustomerRepositoryTrait::new();
                    mock_repo.expect_get_customer_by_username()
                    .times(1)
                    .returning(|username| {
                        Ok(vec![Customer::new(username)])
                    });
                    mock_repo
                };

                let svc = {
                    let customer_repo = Arc::new(RwLock::new(mock_customer_repo));
                    let used_token_repo = Arc::new(RwLock::new(mock_used_token_repo));
                    CustomerServiceImpl::new(fake_issue_at, customer_repo, used_token_repo)
                };

                svc
            },
            CustomerTestContextExpectedResult::WithCustomerResult(Ok(Customer::new("mikejiang"))),
        ),
        CustomerSvcTestContext::new(
            Customer::new("mikejiang"),
            || {
                let mock_used_token_repo = {
                    let mut mock_repo = MockUsedTokenRepositoryTrait::new();
                    mock_repo
                };

                let mock_customer_repo = {
                    let mut mock_repo = MockCustomerRepositoryTrait::new();
                    mock_repo.expect_get_customer_by_username()
                    .times(1)
                    .returning(|username| {
                        Ok(vec![])
                    });
                    mock_repo
                };

                let svc = {
                    let customer_repo = Arc::new(RwLock::new(mock_customer_repo));
                    let used_token_repo = Arc::new(RwLock::new(mock_used_token_repo));
                    CustomerServiceImpl::new(fake_issue_at, customer_repo, used_token_repo)
                };

                svc
            },
            CustomerTestContextExpectedResult::WithCustomerResult(Err(CustomerError::CustomerNotFound)),
        ),
    ];

    for t in test_context {
        let svc = (t.setup_fn)();
        let result = svc
            .get_customer_by_username(&t.input_customer.get_username())
            .await
            .map_err(|err| err.downcast().unwrap());

        let CustomerTestContextExpectedResult::WithCustomerResult(expected_result) = t.expected else {return;};
        assert_eq!(result, expected_result);
    }
}