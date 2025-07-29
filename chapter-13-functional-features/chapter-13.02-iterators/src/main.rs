#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    // Immutable refs iterators: iter()
    // Mutable refs iterators: iter_mut()
    // Returns owned values: into_iter()

    // Some examples of methods in Iterator trait:

    // Sum:
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    // Mapping:
    // the Iterator::map method returns a Map which must be consumed
    // to generate values.
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    // Filtering:
    fn shoes_in_size(
        shoes: Vec<Shoe>,
        shoe_size: u32
    ) -> Vec<Shoe> {
        shoes.into_iter()
            .filter(|s| s.size == shoe_size)
            .collect()
    }

    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal")  },
        Shoe { size: 10, style: String::from("boot")    },
    ];

    let in_my_size = shoes_in_size(shoes, 10);
    println!("in_my_size = {in_my_size:?}");
}
