use mysql::{from_row, params};

use crate::schemas::product::Product;
use crate::schemas::root::Context;

/// User
#[derive(Default, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub test1: String,
    pub test2: String,
    pub test3: String,
    pub test4: String,
    pub test5: String,
    pub test6: String,
    pub test7: String,
    pub test8: String,
    pub test9: String,

    pub test10: String,
    pub test11: String,
    pub test12: String,
    pub test13: String,
    pub test14: String,
    pub test15: String,
    pub test16: String,
    pub test17: String,
    pub test18: String,
    pub test19: String,

    pub test20: String,
    pub test21: String,
    pub test22: String,
    pub test23: String,
    pub test24: String,
    pub test25: String,
    pub test26: String,
    pub test27: String,
    pub test28: String,
    pub test29: String,

    pub test30: String,
    pub test31: String,
    pub test32: String,
    pub test33: String,
    pub test34: String,
    pub test35: String,
    pub test36: String,
    pub test37: String,
    pub test38: String,
    pub test39: String,

    pub test40: String,
    pub test41: String,
    pub test42: String,
    pub test43: String,
    pub test44: String,
    pub test45: String,
    pub test46: String,
    pub test47: String,
    pub test48: String,
    pub test49: String,

    pub test50: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "User Input")]
pub struct UserInput {
    pub name: String,
    pub email: String,
    pub test1: String,
    pub test2: String,
    pub test3: String,
    pub test4: String,
    pub test5: String,
    pub test6: String,
    pub test7: String,
    pub test8: String,
    pub test9: String,

    pub test10: String,
    pub test11: String,
    pub test12: String,
    pub test13: String,
    pub test14: String,
    pub test15: String,
    pub test16: String,
    pub test17: String,
    pub test18: String,
    pub test19: String,

    pub test20: String,
    pub test21: String,
    pub test22: String,
    pub test23: String,
    pub test24: String,
    pub test25: String,
    pub test26: String,
    pub test27: String,
    pub test28: String,
    pub test29: String,

    pub test30: String,
    pub test31: String,
    pub test32: String,
    pub test33: String,
    pub test34: String,
    pub test35: String,
    pub test36: String,
    pub test37: String,
    pub test38: String,
    pub test39: String,

    pub test40: String,
    pub test41: String,
    pub test42: String,
    pub test43: String,
    pub test44: String,
    pub test45: String,
    pub test46: String,
    pub test47: String,
    pub test48: String,
    pub test49: String,

    pub test50: String,
}

#[juniper::graphql_object(Context = Context)]
impl User {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn email(&self) -> &str {
        &self.email
    }

    fn test1(&self) -> &str {
        &self.test1
    }
    fn test2(&self) -> &str {
        &self.test2
    }
    fn test3(&self) -> &str {
        &self.test3
    }
    fn test4(&self) -> &str {
        &self.test4
    }
    fn test5(&self) -> &str {
        &self.test5
    }
    fn test6(&self) -> &str {
        &self.test6
    }
    fn test7(&self) -> &str {
        &self.test7
    }
    fn test8(&self) -> &str {
        &self.test8
    }
    fn test9(&self) -> &str {
        &self.test9
    }

    fn test10(&self) -> &str {
        &self.test10
    }
    fn test11(&self) -> &str {
        &self.test11
    }
    fn test12(&self) -> &str {
        &self.test12
    }
    fn test13(&self) -> &str {
        &self.test13
    }
    fn test14(&self) -> &str {
        &self.test14
    }
    fn test15(&self) -> &str {
        &self.test15
    }
    fn test16(&self) -> &str {
        &self.test16
    }
    fn test17(&self) -> &str {
        &self.test17
    }
    fn test18(&self) -> &str {
        &self.test18
    }
    fn test19(&self) -> &str {
        &self.test19
    }

    fn test20(&self) -> &str {
        &self.test20
    }
    fn test21(&self) -> &str {
        &self.test21
    }
    fn test22(&self) -> &str {
        &self.test22
    }
    fn test23(&self) -> &str {
        &self.test23
    }
    fn test24(&self) -> &str {
        &self.test24
    }
    fn test25(&self) -> &str {
        &self.test25
    }
    fn test26(&self) -> &str {
        &self.test26
    }
    fn test27(&self) -> &str {
        &self.test27
    }
    fn test28(&self) -> &str {
        &self.test28
    }
    fn test29(&self) -> &str {
        &self.test29
    }

    fn test30(&self) -> &str {
        &self.test30
    }
    fn test31(&self) -> &str {
        &self.test31
    }
    fn test32(&self) -> &str {
        &self.test32
    }
    fn test33(&self) -> &str {
        &self.test33
    }
    fn test34(&self) -> &str {
        &self.test34
    }
    fn test35(&self) -> &str {
        &self.test35
    }
    fn test36(&self) -> &str {
        &self.test36
    }
    fn test37(&self) -> &str {
        &self.test37
    }
    fn test38(&self) -> &str {
        &self.test38
    }
    fn test39(&self) -> &str {
        &self.test39
    }

    fn test40(&self) -> &str {
        &self.test40
    }
    fn test41(&self) -> &str {
        &self.test41
    }
    fn test42(&self) -> &str {
        &self.test42
    }
    fn test43(&self) -> &str {
        &self.test43
    }
    fn test44(&self) -> &str {
        &self.test44
    }
    fn test45(&self) -> &str {
        &self.test45
    }
    fn test46(&self) -> &str {
        &self.test46
    }
    fn test47(&self) -> &str {
        &self.test47
    }
    fn test48(&self) -> &str {
        &self.test48
    }
    fn test49(&self) -> &str {
        &self.test49
    }

    fn test50(&self) -> &str {
        &self.test50
    }

    fn products(&self, context: &Context) -> Vec<Product> {
        let mut conn = context.dbpool.get().unwrap();

        conn.prep_exec(
            "SELECT * FROM product WHERE user_id=:user_id",
            params! {
                "user_id" => &self.id
            },
        )
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
        .unwrap()
    }
}
