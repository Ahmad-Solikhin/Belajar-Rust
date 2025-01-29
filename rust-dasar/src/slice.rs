/**
SLice
Slice adalah reference ke sebagian element dari data collection (misal array)
Karena slice reference, jadi dia tidak ada ownernya
Contoh ada array dengan data 10, mau diambil 5 data pertama, bisa dibuat slice sebagai reference dari data ke-1 sampai ke-5

Range
Saat mengambil data di collection harus ditentukan range untuk slicenya
Ada exclusive mulai dari start sampai n-1, dan inclusive start sampai n


*/


#[test]
fn slice_reference() {
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice1: &[i32] = &array[..];
    println!("{:?}", slice1);

    let slice2: &[i32] = &array[0..=5];
    println!("{:?}", slice2);

    let slice3: &[i32] = &array[5..];
    println!("{:?}", slice3);
}

