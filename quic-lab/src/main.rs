fn main() -> Result<(), quiche::Error> {
    let mut config = quiche::Config::new(quiche::PROTOCOL_VERSION)?;
    let server_name = "quic.tech";
    let scid = [0xba; 16];
    /// client
    let conn = quiche::connect(Some(&server_name), &scid, &mut config)?;
    /// server
    // let conn = quiche::accept(&scid, None, &mut config)?;
    /// stats:recv=0 sent=0 lost=0 rtt=333ms cwnd=14520 delivery_rate=0
    /// 往返时延(Round-Trip Time,RTT) 从发送端发送一段数据开始，到发送端收到来自接收端的确认总共经历的时延。
    /// RTT由三个部分决定：链路的传播时间，端系统的处理时间，路由器的缓存中的排队和处理时间。
    /// 拥塞窗口,congestion window,cwnd,它反映了网络的拥塞情况，发送端利用拥塞窗口根据网络的拥塞情况调整数据的发送速度
    println!("{:?}", conn.stats());
    Ok::<(), quiche::Error>(())
}
