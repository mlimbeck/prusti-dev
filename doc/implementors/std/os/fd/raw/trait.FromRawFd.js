(function() {var implementors = {};
implementors["async_std"] = [{"text":"impl <a class=\"trait\" href=\"async_std/os/unix/io/trait.FromRawFd.html\" title=\"trait async_std::os::unix::io::FromRawFd\">FromRawFd</a> for <a class=\"struct\" href=\"async_std/os/unix/net/struct.UnixDatagram.html\" title=\"struct async_std::os::unix::net::UnixDatagram\">UnixDatagram</a>","synthetic":false,"types":["async_std::os::unix::net::datagram::UnixDatagram"]},{"text":"impl <a class=\"trait\" href=\"async_std/os/unix/io/trait.FromRawFd.html\" title=\"trait async_std::os::unix::io::FromRawFd\">FromRawFd</a> for <a class=\"struct\" href=\"async_std/os/unix/net/struct.UnixListener.html\" title=\"struct async_std::os::unix::net::UnixListener\">UnixListener</a>","synthetic":false,"types":["async_std::os::unix::net::listener::UnixListener"]},{"text":"impl <a class=\"trait\" href=\"async_std/os/unix/io/trait.FromRawFd.html\" title=\"trait async_std::os::unix::io::FromRawFd\">FromRawFd</a> for <a class=\"struct\" href=\"async_std/os/unix/net/struct.UnixStream.html\" title=\"struct async_std::os::unix::net::UnixStream\">UnixStream</a>","synthetic":false,"types":["async_std::os::unix::net::stream::UnixStream"]},{"text":"impl <a class=\"trait\" href=\"async_std/os/unix/io/trait.FromRawFd.html\" title=\"trait async_std::os::unix::io::FromRawFd\">FromRawFd</a> for <a class=\"struct\" href=\"async_std/fs/struct.File.html\" title=\"struct async_std::fs::File\">File</a>","synthetic":false,"types":["async_std::fs::file::File"]},{"text":"impl <a class=\"trait\" href=\"async_std/os/unix/io/trait.FromRawFd.html\" title=\"trait async_std::os::unix::io::FromRawFd\">FromRawFd</a> for <a class=\"struct\" href=\"async_std/net/struct.TcpListener.html\" title=\"struct async_std::net::TcpListener\">TcpListener</a>","synthetic":false,"types":["async_std::net::tcp::listener::TcpListener"]},{"text":"impl <a class=\"trait\" href=\"async_std/os/unix/io/trait.FromRawFd.html\" title=\"trait async_std::os::unix::io::FromRawFd\">FromRawFd</a> for <a class=\"struct\" href=\"async_std/net/struct.TcpStream.html\" title=\"struct async_std::net::TcpStream\">TcpStream</a>","synthetic":false,"types":["async_std::net::tcp::stream::TcpStream"]},{"text":"impl <a class=\"trait\" href=\"async_std/os/unix/io/trait.FromRawFd.html\" title=\"trait async_std::os::unix::io::FromRawFd\">FromRawFd</a> for <a class=\"struct\" href=\"async_std/net/struct.UdpSocket.html\" title=\"struct async_std::net::UdpSocket\">UdpSocket</a>","synthetic":false,"types":["async_std::net::udp::UdpSocket"]}];
implementors["mio"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/os/fd/raw/trait.FromRawFd.html\" title=\"trait std::os::fd::raw::FromRawFd\">FromRawFd</a> for <a class=\"struct\" href=\"mio/unix/pipe/struct.Sender.html\" title=\"struct mio::unix::pipe::Sender\">Sender</a>","synthetic":false,"types":["mio::sys::unix::pipe::Sender"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/os/fd/raw/trait.FromRawFd.html\" title=\"trait std::os::fd::raw::FromRawFd\">FromRawFd</a> for <a class=\"struct\" href=\"mio/unix/pipe/struct.Receiver.html\" title=\"struct mio::unix::pipe::Receiver\">Receiver</a>","synthetic":false,"types":["mio::sys::unix::pipe::Receiver"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/os/fd/raw/trait.FromRawFd.html\" title=\"trait std::os::fd::raw::FromRawFd\">FromRawFd</a> for <a class=\"struct\" href=\"mio/net/struct.TcpListener.html\" title=\"struct mio::net::TcpListener\">TcpListener</a>","synthetic":false,"types":["mio::net::tcp::listener::TcpListener"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/os/fd/raw/trait.FromRawFd.html\" title=\"trait std::os::fd::raw::FromRawFd\">FromRawFd</a> for <a class=\"struct\" href=\"mio/net/struct.TcpStream.html\" title=\"struct mio::net::TcpStream\">TcpStream</a>","synthetic":false,"types":["mio::net::tcp::stream::TcpStream"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/os/fd/raw/trait.FromRawFd.html\" title=\"trait std::os::fd::raw::FromRawFd\">FromRawFd</a> for <a class=\"struct\" href=\"mio/net/struct.UdpSocket.html\" title=\"struct mio::net::UdpSocket\">UdpSocket</a>","synthetic":false,"types":["mio::net::udp::UdpSocket"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/os/fd/raw/trait.FromRawFd.html\" title=\"trait std::os::fd::raw::FromRawFd\">FromRawFd</a> for <a class=\"struct\" href=\"mio/net/struct.UnixDatagram.html\" title=\"struct mio::net::UnixDatagram\">UnixDatagram</a>","synthetic":false,"types":["mio::net::uds::datagram::UnixDatagram"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/os/fd/raw/trait.FromRawFd.html\" title=\"trait std::os::fd::raw::FromRawFd\">FromRawFd</a> for <a class=\"struct\" href=\"mio/net/struct.UnixListener.html\" title=\"struct mio::net::UnixListener\">UnixListener</a>","synthetic":false,"types":["mio::net::uds::listener::UnixListener"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/os/fd/raw/trait.FromRawFd.html\" title=\"trait std::os::fd::raw::FromRawFd\">FromRawFd</a> for <a class=\"struct\" href=\"mio/net/struct.UnixStream.html\" title=\"struct mio::net::UnixStream\">UnixStream</a>","synthetic":false,"types":["mio::net::uds::stream::UnixStream"]}];
implementors["socket2"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/os/fd/raw/trait.FromRawFd.html\" title=\"trait std::os::fd::raw::FromRawFd\">FromRawFd</a> for <a class=\"struct\" href=\"socket2/struct.Socket.html\" title=\"struct socket2::Socket\">Socket</a>","synthetic":false,"types":["socket2::socket::Socket"]}];
implementors["tokio"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/os/fd/raw/trait.FromRawFd.html\" title=\"trait std::os::fd::raw::FromRawFd\">FromRawFd</a> for <a class=\"struct\" href=\"tokio/fs/struct.File.html\" title=\"struct tokio::fs::File\">File</a>","synthetic":false,"types":["tokio::fs::file::File"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/os/fd/raw/trait.FromRawFd.html\" title=\"trait std::os::fd::raw::FromRawFd\">FromRawFd</a> for <a class=\"struct\" href=\"tokio/net/struct.TcpSocket.html\" title=\"struct tokio::net::TcpSocket\">TcpSocket</a>","synthetic":false,"types":["tokio::net::tcp::socket::TcpSocket"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()