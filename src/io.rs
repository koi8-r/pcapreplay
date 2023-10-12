use tokio::io::AsyncReadExt ;


pub async fn input(sz: usize) -> std::io::Result<Vec<u8>> {
    let mut all_size = 0usize ;
    let mut buf:Vec<u8> = vec![0; sz] ;
    while all_size < sz {
        let r = match tokio::io::stdin().read(&mut buf).await {
            Ok(n) => {
                all_size += n ;
                Ok(all_size)
            }
            Err(e) => Err(e)
        } ;
        if r.is_err() { return Err(r.err().unwrap()) }
    }
    return Ok(buf)
}
