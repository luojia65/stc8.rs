fn main() {
    use ihex::reader::Reader;
    let rd = Reader::new(
":0300000002011DDD
:0300C30002010037
:10010000C0E0C082C083C0BA75BA8090FE82E0545D
:10011000BFF0B290D0BAD083D082D0E03275813FA8
:1001200075BA8074C090FE80F0748090FE81F07586
:10013000BA00D2AF75BA80748190FE81F075BA00B2
:0201400080FE3F
:00000001FF");
    for line in rd {
        println!("{:?}", line);
    }
}
