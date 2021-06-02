use juniper::{EmptySubscription, FieldError, FieldResult, RootNode};
use mysql::{from_row, params, Error as DBError, Row};

use crate::db::Pool;

use super::product::{Product, ProductInput};
use super::user::User;

pub struct Context {
    pub dbpool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all users")]
    fn users(
        context: &Context,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> FieldResult<Vec<User>> {
        // println!("{}", context);
        let limit = limit.unwrap_or(10);
        let offset = offset.unwrap_or(1);

        let mut conn = context.dbpool.get().unwrap();
        let users = conn
            .prep_exec(
                "select * from user limit :limit offset :offset",
                params! {
                    "limit" => &limit,
                    "offset"=> &offset
                },
            )
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|mut row| {
                        let id = row.take("id").unwrap();
                        let name = row.take("name").unwrap();
                        let email = row.take("email").unwrap();
                        let test1 = row.take("test1").unwrap();
                        let test2 = row.take("test2").unwrap();
                        let test3 = row.take("test3").unwrap();
                        let test4 = row.take("test4").unwrap();
                        let test5 = row.take("test5").unwrap();
                        let test6 = row.take("test6").unwrap();
                        let test7 = row.take("test7").unwrap();
                        let test8 = row.take("test8").unwrap();
                        let test9 = row.take("test9").unwrap();
                        let test10 = row.take("test10").unwrap();
                        let test11 = row.take("test11").unwrap();
                        let test12 = row.take("test12").unwrap();
                        let test13 = row.take("test13").unwrap();
                        let test14 = row.take("test14").unwrap();
                        let test15 = row.take("test15").unwrap();
                        let test16 = row.take("test16").unwrap();
                        let test17 = row.take("test17").unwrap();
                        let test18 = row.take("test18").unwrap();
                        let test19 = row.take("test19").unwrap();
                        let test20 = row.take("test20").unwrap();
                        let test21 = row.take("test21").unwrap();
                        let test22 = row.take("test22").unwrap();
                        let test23 = row.take("test23").unwrap();
                        let test24 = row.take("test24").unwrap();
                        let test25 = row.take("test25").unwrap();
                        let test26 = row.take("test26").unwrap();
                        let test27 = row.take("test27").unwrap();
                        let test28 = row.take("test28").unwrap();
                        let test29 = row.take("test29").unwrap();
                        let test30 = row.take("test30").unwrap();
                        let test31 = row.take("test31").unwrap();
                        let test32 = row.take("test32").unwrap();
                        let test33 = row.take("test33").unwrap();
                        let test34 = row.take("test34").unwrap();
                        let test35 = row.take("test35").unwrap();
                        let test36 = row.take("test36").unwrap();
                        let test37 = row.take("test37").unwrap();
                        let test38 = row.take("test38").unwrap();
                        let test39 = row.take("test39").unwrap();
                        let test40 = row.take("test40").unwrap();
                        let test41 = row.take("test41").unwrap();
                        let test42 = row.take("test42").unwrap();
                        let test43 = row.take("test43").unwrap();
                        let test44 = row.take("test44").unwrap();
                        let test45 = row.take("test45").unwrap();
                        let test46 = row.take("test46").unwrap();
                        let test47 = row.take("test47").unwrap();
                        let test48 = row.take("test48").unwrap();
                        let test49 = row.take("test49").unwrap();
                        let test50 = row.take("test50").unwrap();
                        User {
                            id,
                            name,
                            email,
                            test1,
                            test2,
                            test3,
                            test4,
                            test5,
                            test6,
                            test7,
                            test8,
                            test9,
                            test10,
                            test11,
                            test12,
                            test13,
                            test14,
                            test15,
                            test16,
                            test17,
                            test18,
                            test19,
                            test20,
                            test21,
                            test22,
                            test23,
                            test24,
                            test25,
                            test26,
                            test27,
                            test28,
                            test29,
                            test30,
                            test31,
                            test32,
                            test33,
                            test34,
                            test35,
                            test36,
                            test37,
                            test38,
                            test39,
                            test40,
                            test41,
                            test42,
                            test43,
                            test44,
                            test45,
                            test46,
                            test47,
                            test48,
                            test49,
                            test50,
                        }
                    })
                    .collect()
            })
            .unwrap();
        Ok(users)
    }

    #[graphql(description = "Get Single user reference by user ID")]
    fn user(context: &Context, id: String) -> FieldResult<User> {
        let mut conn = context.dbpool.get().unwrap();

        let user: Result<Option<Row>, DBError> =
            conn.first_exec("SELECT * FROM user WHERE id=:id", params! {"id" => id});

        if let Err(_err) = user {
            return Err(FieldError::new(
                "User Not Found",
                graphql_value!({ "not_found": "user not found" }),
            ));
        }
        let mut row = user.unwrap().unwrap();
        let id = row.take("id").unwrap();
        let name = row.take("name").unwrap();
        let email = row.take("email").unwrap();
        let test1 = row.take("test1").unwrap();
        let test2 = row.take("test2").unwrap();
        let test3 = row.take("test3").unwrap();
        let test4 = row.take("test4").unwrap();
        let test5 = row.take("test5").unwrap();
        let test6 = row.take("test6").unwrap();
        let test7 = row.take("test7").unwrap();
        let test8 = row.take("test8").unwrap();
        let test9 = row.take("test9").unwrap();
        let test10 = row.take("test10").unwrap();
        let test11 = row.take("test11").unwrap();
        let test12 = row.take("test12").unwrap();
        let test13 = row.take("test13").unwrap();
        let test14 = row.take("test14").unwrap();
        let test15 = row.take("test15").unwrap();
        let test16 = row.take("test16").unwrap();
        let test17 = row.take("test17").unwrap();
        let test18 = row.take("test18").unwrap();
        let test19 = row.take("test19").unwrap();
        let test20 = row.take("test20").unwrap();
        let test21 = row.take("test21").unwrap();
        let test22 = row.take("test22").unwrap();
        let test23 = row.take("test23").unwrap();
        let test24 = row.take("test24").unwrap();
        let test25 = row.take("test25").unwrap();
        let test26 = row.take("test26").unwrap();
        let test27 = row.take("test27").unwrap();
        let test28 = row.take("test28").unwrap();
        let test29 = row.take("test29").unwrap();
        let test30 = row.take("test30").unwrap();
        let test31 = row.take("test31").unwrap();
        let test32 = row.take("test32").unwrap();
        let test33 = row.take("test33").unwrap();
        let test34 = row.take("test34").unwrap();
        let test35 = row.take("test35").unwrap();
        let test36 = row.take("test36").unwrap();
        let test37 = row.take("test37").unwrap();
        let test38 = row.take("test38").unwrap();
        let test39 = row.take("test39").unwrap();
        let test40 = row.take("test40").unwrap();
        let test41 = row.take("test41").unwrap();
        let test42 = row.take("test42").unwrap();
        let test43 = row.take("test43").unwrap();
        let test44 = row.take("test44").unwrap();
        let test45 = row.take("test45").unwrap();
        let test46 = row.take("test46").unwrap();
        let test47 = row.take("test47").unwrap();
        let test48 = row.take("test48").unwrap();
        let test49 = row.take("test49").unwrap();
        let test50 = row.take("test50").unwrap();

        Ok(User {
            id,
            name,
            email,
            test1,
            test2,
            test3,
            test4,
            test5,
            test6,
            test7,
            test8,
            test9,
            test10,
            test11,
            test12,
            test13,
            test14,
            test15,
            test16,
            test17,
            test18,
            test19,
            test20,
            test21,
            test22,
            test23,
            test24,
            test25,
            test26,
            test27,
            test28,
            test29,
            test30,
            test31,
            test32,
            test33,
            test34,
            test35,
            test36,
            test37,
            test38,
            test39,
            test40,
            test41,
            test42,
            test43,
            test44,
            test45,
            test46,
            test47,
            test48,
            test49,
            test50,
        })
    }

    #[graphql(description = "List of all users")]
    fn products(context: &Context) -> FieldResult<Vec<Product>> {
        let mut conn = context.dbpool.get().unwrap();
        let products = conn
            .prep_exec("select * from product", ())
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (id, user_id, name, price) = from_row(row);
                        Product {
                            id,
                            user_id,
                            name,
                            price,
                        }
                    })
                    .collect()
            })
            .unwrap();
        Ok(products)
    }

    #[graphql(description = "Get Single user reference by user ID")]
    fn product(context: &Context, id: String) -> FieldResult<Product> {
        let mut conn = context.dbpool.get().unwrap();
        let product: Result<Option<Row>, DBError> =
            conn.first_exec("SELECT * FROM user WHERE id=:id", params! {"id" => id});
        if let Err(_err) = product {
            return Err(FieldError::new(
                "Product Not Found",
                graphql_value!({ "not_found": "product not found" }),
            ));
        }

        let (id, user_id, name, price) = from_row(product.unwrap().unwrap());
        Ok(Product {
            id,
            user_id,
            name,
            price,
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    fn create_user(context: &Context) -> FieldResult<User> {
        let mut conn = context.dbpool.get().unwrap();
        // let new_id = uuid::Uuid::new_v4().to_simple().to_string();

        // let insert: Result<Option<Row>, DBError> = conn.first_exec(
        //     "INSERT INTO user(id, name, email) VALUES(:id, :name, :email)",
        //     params! {
        //         "id" => &new_id,
        //         "name" => &user.name,
        //         "email" => &user.email,
        //     },
        // );
        let instant1 = std::time::Instant::now();
        std::thread::sleep(std::time::Duration::from_millis(200));

        for i in 0..100000 {
            let new_id = uuid::Uuid::new_v4().to_simple().to_string();

            let email = format!("testemail.{}@example.com", i);
            let insert: Result<Option<Row>, DBError> = conn.first_exec(
                "INSERT INTO `user` (`id`, `name`, `email`, `test1`, `test2`, `test3`, `test4`, `test5`, `test6`, `test7`, `test8`, `test9`, `test10`, `test11`, `test12`, `test13`, `test14`, `test15`, `test16`, `test17`, `test18`, `test19`, `test20`, `test21`, `test22`, `test23`, `test24`, `test25`, `test26`, `test27`, `test28`, `test29`, `test30`, `test31`, `test32`, `test33`, `test34`, `test35`, `test36`, `test37`, `test38`, `test39`, `test40`, `test41`, `test42`, `test43`, `test44`, `test45`, `test46`, `test47`, `test48`, `test49`, `test50`) VALUES (:uuid, 'name', :email, 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>', 'test test @ test test <tag>test</tag>');",
                params! {
                    "uuid" => &new_id,
                    "email" => &email,
                },
            );
            match insert {
                Ok(_opt_row) => {}
                Err(err) => {
                    let msg = match err {
                        DBError::MySqlError(err) => err.message,
                        _ => "internal error".to_owned(),
                    };
                    println!("{}", msg)
                }
            };
        }
        let instant2 = std::time::Instant::now();
        eprintln!("elapsed: {:?}", instant2 - instant1);
        let new_id = uuid::Uuid::new_v4().to_simple().to_string();

        let insert: Result<Option<Row>, DBError> = conn.first_exec(
            "INSERT INTO user(id, name, email) VALUES(:id, :name, :email)",
            params! {
                "id" => &new_id,
                "name" => "test".to_string(),
                "email" => "test".to_string(),
            },
        );
        match insert {
            Ok(_opt_row) => Ok(User {
                id: new_id,
                name: "test".to_string(),
                email: "test@test".to_string(),
                test1: "test".to_string(),
                test2: "test".to_string(),
                test3: "test".to_string(),
                test4: "test".to_string(),
                test5: "test".to_string(),
                test6: "test".to_string(),
                test7: "test".to_string(),
                test8: "test".to_string(),
                test9: "test".to_string(),
                test10: "test".to_string(),
                test11: "test".to_string(),
                test12: "test".to_string(),
                test13: "test".to_string(),
                test14: "test".to_string(),
                test15: "test".to_string(),
                test16: "test".to_string(),
                test17: "test".to_string(),
                test18: "test".to_string(),
                test19: "test".to_string(),
                test20: "test".to_string(),
                test21: "test".to_string(),
                test22: "test".to_string(),
                test23: "test".to_string(),
                test24: "test".to_string(),
                test25: "test".to_string(),
                test26: "test".to_string(),
                test27: "test".to_string(),
                test28: "test".to_string(),
                test29: "test".to_string(),
                test30: "test".to_string(),
                test31: "test".to_string(),
                test32: "test".to_string(),
                test33: "test".to_string(),
                test34: "test".to_string(),
                test35: "test".to_string(),
                test36: "test".to_string(),
                test37: "test".to_string(),
                test38: "test".to_string(),
                test39: "test".to_string(),
                test40: "test".to_string(),
                test41: "test".to_string(),
                test42: "test".to_string(),
                test43: "test".to_string(),
                test44: "test".to_string(),
                test45: "test".to_string(),
                test46: "test".to_string(),
                test47: "test".to_string(),
                test48: "test".to_string(),
                test49: "test".to_string(),
                test50: "test".to_string(),
            }),
            Err(err) => {
                let msg = match err {
                    DBError::MySqlError(err) => err.message,
                    _ => "internal error".to_owned(),
                };
                Err(FieldError::new(
                    "Failed to create new user.",
                    graphql_value!({ "internal_error": msg }),
                ))
            }
        }
    }

    fn create_product(context: &Context, product: ProductInput) -> FieldResult<Product> {
        let mut conn = context.dbpool.get().unwrap();
        let new_id = uuid::Uuid::new_v4().to_simple().to_string();

        let insert: Result<Option<Row>, DBError> = conn.first_exec(
            "INSERT INTO product(id, user_id, name, price) VALUES(:id, :user_id, :name, :price)",
            params! {
                "id" => &new_id,
                "user_id" => &product.user_id,
                "name" => &product.name,
                "price" => &product.price.to_owned(),
            },
        );

        match insert {
            Ok(_opt_row) => Ok(Product {
                id: new_id,
                user_id: product.user_id,
                name: product.name,
                price: product.price,
            }),
            Err(err) => {
                let msg = match err {
                    DBError::MySqlError(err) => err.message,
                    _ => "internal error".to_owned(),
                };
                Err(FieldError::new(
                    "Failed to create new product",
                    graphql_value!({ "internal_error": msg }),
                ))
            }
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
