
TODO: 어떻게 패킷 최대 길이를 정할지 생각해보기: 쓰레드별로 

---
1. TCPListener
2. HandShake
3. Status
4. Login
5. Configuration 
6. Play
---

TODO: String 네트워크 타입의 앞부분에 있는 VarInt는 왜 최대 길이가 5인 타입인데 String 네트워크 타입의 앞부분일 때는 최대 길이가 3이라고 명시되어 있는지에 대한 궁금증을 해결하기 위해서 Mintlin에 구현된 코드를 읽어보자.

# 패킷 최대 길이 

|---|---|
|이름|패킷최대 길이|
|TcpListener| - |
|HandShake| 780 |
|Status|아직 안세봄|
|Login|아직 안세봄|
|Conf/Play|3바이트 최대 길이 및 위키에 써있는 기본적인 패킷 길이값|

# 소켓 셀렉터 쓰레드별 추가 설명

> TcpListener랑 HandShake/Login은(는) 분리해봤자 별 의미가 없지 않나?
ㅇㅇ 그럴듯 굳이 분리할 필요가 없음
TODO: selector registry에 accept가 아닌 read만 있을 때 성능 향상이 있는지 확인해볼것! 

> Configuration이랑 Play는 또 왜 굳이 분리하고?
적어도 이건 handshake 또는 login이랑 다른 쓰레드에서 동작해야 하잖아.
