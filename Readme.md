# Kidsnote backup

키즈노트 백업 프로그램
- 알림장 이미지를 다운로드 받습니다.
- 알림장 텍스트를 이미지로 변환해서 일괄 다운로드 받습니다.

## help otpions

```
knbackup login --help

Usage: knbackup login [OPTIONS]

Options:
  -c, --client_id <Client id>          Client ID [env: KNB_CLIENT_ID=]
      --debug                          
  -u, --user <User ID>                 UserID of the Account to greet [env: KNB_USER_ID=]
  -p, --pass <User Password>           Password of the Account to greet [env: KNB_USER_PASS=]
  -r, --refresh-token <REFRESH_TOKEN>  RefreshToken of the Account to greet
      --config <Config File Path>      [default: ~/.knbackup/config.toml]
  -h, --help                           Print help
```

```
knbackup download --help

Usage: knbackup download [OPTIONS]

Options:
  -c, --client_id <Client id>          Client ID [env: KNB_CLIENT_ID=]
      --debug                          
  -u, --user <User ID>                 UserID of the Account to greet [env: KNB_USER_ID=]
  -p, --pass <User Password>           Password of the Account to greet [env: KNB_USER_PASS=]
  -r, --refresh-token <REFRESH_TOKEN>  RefreshToken of the Account to greet
      --config <Config File Path>      [default: ~/.knbackup/config.toml]
      --date-start <Start Date>        Backup start date
      --date-end <End Date>            Backup end date
  -o, --output-path <Output Path>      [default: ./output]
  -t, --test                           
  -h, --help                           Print help
```

## Example

### Login

- 최초 로그인하면 로그인 정보를 --config 경로에 저장합니다.
- 기본값은 ~/.knbackup/config.toml 입니다.

password Login 
```
knbackup login -u user_id -p password
```

refresh token login
```
knbackup login -r refresh_token
```

### Download

- login 옵션을 사용하지 않는 경우 --config 의 로그인 정보를 사용합니다.
- 다운로드 받은 파일은 --output 경로에 생섭됩니다.
  - 기본값은 ./output 입니다.
  - ex) /[2023] 키즈노트어린이집 홍길동/[2023-01] 키즈노트어린이집/키즈노트어린이집_알림장_20230101_홍길동_xxx.jpg

login 되어 있는 경우 (로그인 매개변수 생략 가능)
```
knbackup download
```

password login 후 download
```
knbackup download -u user_id -p password
```

refresh_token login 후 download
```
knbackup download -r refresh_token
```

## Build
```
cargo build --release
```

## 해야할 것들
- 알림장의 텍스트에 댓글 텍스트 추가하기
- 알림장의 첨부 비디오 다운로드 받기
- access_token 만료 되었을 경우 refresh 하기