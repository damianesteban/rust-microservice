use super::schema::*;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Local;
use crate::errors::MyStoreError;
use diesel::PgConnection;

/**
 * This is a convenience type for registering the user.
 */
#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
    pub password_confirmation: String
}

impl RegisterUser {
    /** Validates a user */
    pub fn validates(self) ->
     Result<RegisterUser, MyStoreError> {
         if self.password == self.password_confirmation {
             Ok(self)
         } else {
             Err(
                 MyStoreError::PasswordNotMatch(
                     "Password and Password Confirmation does not match".to_string()
                 )
             )
         }
    }
}

// The User model for the database.
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
  pub email: String,
  pub hash: String,
  pub created_at: chrono::NaiveDateTime,
}

impl User {
  pub fn from_details<S: Into<String>, T: Into<String>>(email: S, pwd: T) -> Self {
    User {
      email: email.into(),
      hash: pwd.into(),
      created_at: chrono::Local::now().naive_local(),
    }
  }

  /**
   * Creates a new User and inserts it into the database.
   */
  pub fn create(register_user: RegisterUser, connection: &PgConnection) -> Result<User, MyStoreError> {
    use diesel::RunQueryDsl;

    Ok(diesel::insert_into(users::table)
      .values(User {
        email: register_user.email,
        hash: Self::hash_password(register_user.password)?,
        created_at: Local::now().naive_local()
      })
      .get_result(connection)?)
  }

     // This might look kind of weird, 
    // but if something fails it would chain 
    // to our MyStoreError Error, 
    // otherwise it will gives us the hash, 
    // we still need to return a result 
    // so we wrap it in an Ok variant from the Result type. 
    pub fn hash_password(plain: String) -> Result<String, MyStoreError> {
        Ok(hash(plain, DEFAULT_COST)?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String
}

impl AuthUser {

    // The good thing about ? syntax and have a custom error is 
    // that the code would look very straightforward, I mean, 
    // the other way would imply a lot of pattern matching 
    // making it look ugly. 
    pub fn login(&self, connection: &PgConnection) ->
     Result<User, MyStoreError> {
        use bcrypt::verify;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use diesel::ExpressionMethods;
        use crate::schema::users::dsl::email;

        let mut records =
            users::table
                .filter(email.eq(&self.email))
                .load::<User>(connection)?;

        let user =
            records
                .pop()
                .ok_or(MyStoreError::DBError(diesel::result::Error::NotFound))?;

        let verify_password =
            verify(&self.password, &user.hash)
                .map_err( |_error| {
                    MyStoreError::WrongPassword(
                        "Wrong password, check again please".to_string()
                    )
                })?;

        if verify_password {
            Ok(user)
        } else {
            Err(MyStoreError::WrongPassword(
                "Wrong password, check again please".to_string()
            ))
        }

    }
}
// Convenience type to get a list of users from the database
#[derive(Serialize, Deserialize)]
pub struct UserList(pub Vec<User>);

impl UserList {
  pub fn list(connection: &PgConnection) -> Result<UserList, MyStoreError> {
      // These four statements can be placed in the top, or here, your call.
      use diesel::RunQueryDsl;
      use diesel::QueryDsl;
      use crate::schema::users::dsl::*;


      let result = 
            users
                .limit(10)
                .load::<User>(connection)
                .expect("Error loading users");

        // We return a value by leaving it without a comma
        Ok(UserList(result))
  }
}
