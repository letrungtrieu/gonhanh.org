cargo build
sudo install -Dm755 target/debug/ibus-engine-gonhanh /usr/libexec/ibus-engine-gonhanh
sudo install -Dm644 data/gonhanh.xml /usr/share/ibus/component/gonhanh.xml
ibus restart
