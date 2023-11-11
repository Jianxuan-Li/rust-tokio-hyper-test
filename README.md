# Rust tokio and hyper test

```bash
cargo run

docker build -t freeyeti/hyper-test .
docker push freeyeti/hyper-test
```

```json
{
"time": "2023-11-11 17:08:53",
"headers": [
  {"host": "localhost:3000"},
  {"connection": "keep-alive"},
  {"cache-control": "max-age=0"},
  {"sec-ch-ua": ""Google Chrome";v="119", "Chromium";v="119", "Not?A_Brand";v="24""},
  {"sec-ch-ua-mobile": "?0"},
  {"sec-ch-ua-platform": ""macOS""},
  {"upgrade-insecure-requests": "1"},
  {"user-agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"},
  {"accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"},
  {"sec-fetch-site": "none"},
  {"sec-fetch-mode": "navigate"},
  {"sec-fetch-user": "?1"},
  {"sec-fetch-dest": "document"},
  {"accept-encoding": "gzip, deflate, br"},
  {"accept-language": "en,zh;q=0.9,ko;q=0.8"},
  {"cookie": "wagtail_sidebar_collapsed=0; PGADMIN_LANGUAGE=en; next-auth.csrf-token=1d513b1286b6521a86b47e0231329c973bf2cdbb4dff9d6914705ca016488ed1%7Cf38f9d7266dfb80137a2fb560efc427aef042c245cd55465bbd3e4524dd4a352; next-auth.callback-url=http%3A%2F%2Flocalhost%3A3000; _xsrf=2|8c887cbf|a53aebc6bde6613c885f6432215d833b|1682001511; PHPSESSID=e2348f3c5500da5d3d3cb015543c8d7a; showhints=1; csrftoken=Liltf1K23QE6kuY9IkWul9P6litJLsmo; sessionid=apzb4t7eyih1ixt7ltbeozfokmat0140"},
],
"uri": "/"
,"ip": "127.0.0.1:54010"
}
```
