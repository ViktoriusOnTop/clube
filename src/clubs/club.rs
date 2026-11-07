pub struct Club {
    pub id: u64, //random, static id
    pub created: u64, //created seconds since unix epoch
    pub age: u64, //seconds since unix epoch
    pub name: String, //changeable name
    pub owners: Vec<u64>,
}