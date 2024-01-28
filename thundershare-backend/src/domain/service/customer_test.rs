use std::sync::Arc;

use tokio::sync::RwLock;

use crate::domain::{
    entity::customer::Customer, error::customer::CustomerError,
    repository::customer::MockCustomerRepositoryTrait,
};

use super::customer::{CustomerServiceImpl, CustomerServiceTrait};

struct CustomerSignUpTestContext {
    pub input_customer: Customer,
    pub setup_fn: Box<dyn Fn() -> Arc<dyn CustomerServiceTrait>>,
    pub expected: std::result::Result<Customer, CustomerError>,
}

impl CustomerSignUpTestContext {
    fn new(
        input_customer: Customer,
        setup_fn: impl Fn() -> Arc<dyn CustomerServiceTrait> + 'static,
        expected: std::result::Result<Customer, CustomerError>,
    ) -> CustomerSignUpTestContext {
        CustomerSignUpTestContext {
            input_customer,
            setup_fn: Box::new(setup_fn),
            expected,
        }
    }
}

#[actix_rt::test]
async fn test_customer_svc_signup() {
    let test_context = vec![
        CustomerSignUpTestContext::new(
            Customer::new("mikejiang", ""),
            || {
                let mock_customer_repo = {
                    let mut mock_repo = MockCustomerRepositoryTrait::new();

                    mock_repo
                        .expect_get_customer_by_username()
                        .times(1)
                        .returning(move |_username| Ok(vec![]));

                    mock_repo
                        .expect_create_customer()
                        .times(1)
                        .returning(|username, password| Ok(Customer::new(username, password)));

                    mock_repo
                };

                let svc = {
                    let customer_repo = Arc::new(RwLock::new(mock_customer_repo));
                    CustomerServiceImpl::new(customer_repo)
                };

                svc
            },
            Ok(Customer::new("mikejiang", "")),
        ),
        CustomerSignUpTestContext::new(
            Customer::new("brucewayne", ""),
            || {
                let mock_customer_repo = {
                    let mut mock_repo = MockCustomerRepositoryTrait::new();

                    mock_repo
                        .expect_get_customer_by_username()
                        .times(1)
                        .returning(move |_username| Ok(vec![Customer::new("", "")]));
                    mock_repo
                };

                let svc = {
                    let customer_repo = Arc::new(RwLock::new(mock_customer_repo));
                    CustomerServiceImpl::new(customer_repo)
                };

                svc
            },
            Err(CustomerError::CustomerAlreadyExist),
        ),
    ];

    for t in test_context {
        let svc = (t.setup_fn)();
        let result = svc
            .customer_signup(
                &t.input_customer.get_username(),
                &t.input_customer.get_password(),
            )
            .await
            .map_err(|err| err.downcast().unwrap());

        assert_eq!(result, t.expected);
    }
}

struct CustomerSignInTestContext {
    pub input_customer: Customer,
    pub setup_fn: Box<dyn Fn() -> Arc<dyn CustomerServiceTrait>>,
    pub expected: std::result::Result<Customer, CustomerError>,
}

impl CustomerSignInTestContext {
    fn new(
        input_customer: Customer,
        setup_fn: impl Fn() -> Arc<dyn CustomerServiceTrait> + 'static,
        expected: std::result::Result<Customer, CustomerError>,
    ) -> CustomerSignInTestContext {
        CustomerSignInTestContext {
            input_customer,
            setup_fn: Box::new(setup_fn),
            expected,
        }
    }
}

#[actix_rt::test]
async fn test_customer_svc_signin() {
    let test_context = vec![
        CustomerSignInTestContext::new(
            Customer::new("mikejiang", "password"),
            || {
                let mock_customer_repo = {
                    let mut mock_repo = MockCustomerRepositoryTrait::new();

                    mock_repo
                        .expect_get_customer_by_credential()
                        .times(1)
                        .returning(move |username, password| {
                            Ok(vec![Customer::new(username, password)])
                        });

                    mock_repo
                };

                let svc = {
                    let customer_repo = Arc::new(RwLock::new(mock_customer_repo));
                    CustomerServiceImpl::new(customer_repo)
                };

                svc
            },
            Ok(Customer::new("mikejiang", "password")),
        ),
        CustomerSignInTestContext::new(
            Customer::new("brucewayne", ""),
            || {
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
                    CustomerServiceImpl::new(customer_repo)
                };

                svc
            },
            Err(CustomerError::CustomerInvalidCredential),
        ),
    ];

    for t in test_context {
        let svc = (t.setup_fn)();
        let result = svc
            .customer_signin(
                &t.input_customer.get_username(),
                &t.input_customer.get_password(),
            )
            .await
            .map_err(|err| err.downcast().unwrap());

        assert_eq!(result, t.expected);
    }
}
