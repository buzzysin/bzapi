use crate::models::MyConnection;

pub struct MyDieselAdaptor;
bzauth_rs::adapt_diesel!(
    MyDieselAdaptor,
    MyConnection,
    User = crate::models::user::User,
    UserTable = crate::schema::users,
    Account = crate::models::account::Account,
    AccountTable = crate::schema::accounts,
    Session = crate::models::session::Session,
    SessionTable = crate::schema::sessions,
    VerificationToken = crate::models::verification_token::VerificationToken,
    VerificationTokenTable = crate::schema::verification_tokens,
);
