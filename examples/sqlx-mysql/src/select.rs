use sea_orm::{entity::*, query::*, Database, FromQueryResult};
use super::*;

pub async fn all_about_select(db: &Database) -> Result<(), QueryErr> {
    find_all(db).await?;

    println!("===== =====\n");

    find_together(db).await?;

    println!("===== =====\n");

    find_one(db).await?;

    println!("===== =====\n");

    count_fruits_by_cake(db).await?;

    println!("===== =====\n");

    find_many_to_many(db).await?;

    if false {
        println!("===== =====\n");

        all_about_select_json(db).await?;
    }

    Ok(())
}

async fn find_all(db: &Database) -> Result<(), QueryErr> {
    print!("find all cakes: ");

    let cakes = Cake::find().all(db).await?;

    println!();
    for cc in cakes.iter() {
        println!("{:?}\n", cc);
    }

    print!("find all fruits: ");

    let fruits = Fruit::find().all(db).await?;

    println!();
    for ff in fruits.iter() {
        println!("{:?}\n", ff);
    }

    Ok(())
}

async fn find_together(db: &Database) -> Result<(), QueryErr> {
    print!("find cakes and fruits: ");

    let both = Cake::find()
        .left_join_and_select(Fruit)
        .all(db)
        .await?;

    println!();
    for bb in both.iter() {
        println!("{:?}\n", bb);
    }

    Ok(())
}

async fn find_one(db: &Database) -> Result<(), QueryErr> {
    print!("find one by primary key: ");

    let cheese: Option<cake::Model> = Cake::find_by(1).one(db).await?;
    let cheese = cheese.unwrap();

    println!();
    println!("{:?}", cheese);
    println!();

    print!("find one by like: ");

    let chocolate = Cake::find()
        .filter(cake::Column::Name.contains("chocolate"))
        .one(db)
        .await?;

    println!();
    println!("{:?}", chocolate);
    println!();

    print!("find models belong to: ");

    let fruits = cheese.find_fruit().all(db).await?;

    println!();
    for ff in fruits.iter() {
        println!("{:?}\n", ff);
    }

    Ok(())
}

async fn count_fruits_by_cake(db: &Database) -> Result<(), QueryErr> {
    #[derive(Debug, FromQueryResult)]
    struct SelectResult {
        name: String,
        num_of_fruits: i32,
    }

    print!("count fruits by cake: ");

    let select = Cake::find()
        .left_join(Fruit)
        .select_only()
        .column(cake::Column::Name)
        .column_as(fruit::Column::Id.count(), "num_of_fruits")
        .group_by(cake::Column::Name);

    let results = select.into_model::<SelectResult>().all(db).await?;

    println!();
    for rr in results.iter() {
        println!("{:?}\n", rr);
    }

    Ok(())
}

async fn find_many_to_many(db: &Database) -> Result<(), QueryErr> {
    print!("find cakes and fillings: ");

    let both = Cake::find()
        .left_join_and_select(Filling)
        .all(db)
        .await?;

    println!();
    for bb in both.iter() {
        println!("{:?}\n", bb);
    }

    print!("find fillings for cheese cake: ");

    let cheese = Cake::find_by(1).one(db).await?;

    if let Some(cheese) = cheese {
        let fillings: Vec<filling::Model> = cheese.find_filling().all(db).await?;

        println!();
        for ff in fillings.iter() {
            println!("{:?}\n", ff);
        }
    }

    print!("find cakes for lemon: ");

    let lemon = Filling::find_by(2).one(db).await?;

    if let Some(lemon) = lemon {
        let cakes: Vec<cake::Model> = lemon.find_cake().all(db).await?;

        println!();
        for cc in cakes.iter() {
            println!("{:?}\n", cc);
        }
    }

    Ok(())
}

async fn all_about_select_json(db: &Database) -> Result<(), QueryErr> {
    find_all_json(&db).await?;

    println!("===== =====\n");

    find_together_json(&db).await?;

    println!("===== =====\n");

    count_fruits_by_cake_json(&db).await?;

    Ok(())
}

async fn find_all_json(db: &Database) -> Result<(), QueryErr> {
    print!("find all cakes: ");

    let cakes = Cake::find().into_json().all(db).await?;

    println!("\n{}\n", serde_json::to_string_pretty(&cakes).unwrap());

    print!("find all fruits: ");

    let fruits = Fruit::find().into_json().all(db).await?;

    println!("\n{}\n", serde_json::to_string_pretty(&fruits).unwrap());

    Ok(())
}

async fn find_together_json(db: &Database) -> Result<(), QueryErr> {
    print!("find cakes and fruits: ");

    let cakes_fruits = Cake::find()
        .left_join_and_select(Fruit)
        .into_json()
        .all(db)
        .await?;

    println!(
        "\n{}\n",
        serde_json::to_string_pretty(&cakes_fruits).unwrap()
    );

    Ok(())
}

async fn count_fruits_by_cake_json(db: &Database) -> Result<(), QueryErr> {
    print!("count fruits by cake: ");

    let count = Cake::find()
        .left_join(Fruit)
        .select_only()
        .column(cake::Column::Name)
        .column_as(fruit::Column::Id.count(), "num_of_fruits")
        .group_by(cake::Column::Name)
        .into_json()
        .all(db)
        .await?;

    println!("\n{}\n", serde_json::to_string_pretty(&count).unwrap());

    Ok(())
}