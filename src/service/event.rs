



// pub async fn create(dto: NewEventDto, pool: &PGPool, auth_state: Arc<Mutex<UserAuthState>>) -> Result<u64, MyError> {
//     let NewEventDto { title, descr, dt, place } = dto;
//     let res = db::event::create(Event {
//         id: Uuid::new_v4(), title, descr, dt, place,
//         creator: auth_state.lock().unwrap().id.unwrap(),   
//     }, pool).await;  
//     match res {
//         Ok(val) => Ok(val.rows_affected()),
//         Err(_) => Err(MyError::InternalError)
//     }
// }