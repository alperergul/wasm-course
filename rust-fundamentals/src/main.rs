
fn add(num_one:i32, num_two:i32) -> i32 {

  return num_one + num_two;

}

fn main(){

  let mut total = add(1,1);
  let mut free_shipping = false;
  
  if total > 50 {
    println!("You qualify for free shipping");
    free_shipping = true;
  } else if total > 20{
    println!("If you add more items, you can qualify for free shipping");
  } else {
    println!("No free shipping!")
  }

  match free_shipping {
      true => total = total + 0,
      false => total = total + 5
  }

  match total {
      1 => println!("1"),
      2 => println!("2"),
      _ => println!("No match")
  }

  println!("Total {:?}", total);

  let items: [i32; 5] = [1,2,3,4,5];

  println!("{:?}", items);

  // Arrayler sabit uzunlukta ise kullanılır...
  // Vectorler sabit bir uzunluk yok ise kullanılır...
  let vector_item = vec![1,2,3,4,5,6];
  let mut vector_item_2 = Vec::new();
  vector_item_2.push(1);
  vector_item_2.push(2);
  vector_item_2.push(3);
  vector_item_2.push(4);

  println!("{:?}", vector_item);
  println!("{:?}", vector_item_2);

  
}