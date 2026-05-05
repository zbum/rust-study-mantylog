# mantylog

작은 로그 분석 CLI. 텍스트 로그 파일을 읽어 라인별 레벨(`ERROR`/`WARN`/`INFO`/`UNKNOWN`)을 분류하고, 필터링/카운트/JSON 출력을 지원합니다.

> 이 프로젝트는 **Rust 학습용**으로 만들어졌습니다. 학습 여정과 단계별 진도는 [LEARNING.md](./LEARNING.md)를 참고하세요.

## 설치

Rust 1.95 이상이 필요합니다 ([rustup](https://rustup.rs)).

```bash
git clone https://github.com/zbum/rust-study-mantylog.git
cd rust-study-mantylog
cargo build --release
```

빌드 결과는 `target/release/mantylog`. 또는 `cargo run --` 으로 바로 실행 가능합니다.

## 사용법

```
Usage: mantylog [OPTIONS]

Options:
  -i, --input <INPUT>    입력 로그 파일 경로 [default: sample.log]
  -f, --filter <FILTER>  특정 레벨만 출력 (지정 안하면 전부)
                         [possible values: error, warn, info, unknown]
      --format <FORMAT>  출력 포맷 [default: text] [possible values: text, json]
  -h, --help             Print help
  -V, --version          Print version
```

## 예시

샘플 로그 파일이 함께 포함돼 있습니다 (`sample.log`).

### 전체 분석 (텍스트 모드)

```bash
$ cargo run
[INFO   ] 2026-05-04 10:00:00 INFO  application started
[INFO   ] 2026-05-04 10:00:01 INFO  config loaded from /etc/manty/app.yaml
[WARN   ] 2026-05-04 10:00:02 WARN  slow query 1.2s on users table
[ERROR  ] 2026-05-04 10:00:04 ERROR connection refused to 10.0.0.5:5432
...
--- summary ---
ERROR: 3, WARN: 3, INFO: 8, UNKNOWN: 1
```

### 레벨 필터링

```bash
$ cargo run -- --filter error
[ERROR  ] 2026-05-04 10:00:04 ERROR connection refused to 10.0.0.5:5432
[ERROR  ] 2026-05-04 10:00:07 ERROR null pointer in OrderService.calculate
[ERROR  ] 2026-05-04 10:00:12 ERROR timeout reading from kafka topic events
--- summary ---
ERROR: 3, WARN: 3, INFO: 8, UNKNOWN: 1
```

카운트는 항상 **전체 라인** 기준으로 집계됩니다 (필터는 출력만 거름).

### JSON 출력 (JSONL)

```bash
$ cargo run -- --filter error --format json
{"level":"ERROR","line":"2026-05-04 10:00:04 ERROR connection refused to 10.0.0.5:5432"}
{"level":"ERROR","line":"2026-05-04 10:00:07 ERROR null pointer in OrderService.calculate"}
{"level":"ERROR","line":"2026-05-04 10:00:12 ERROR timeout reading from kafka topic events"}
```

### 파이프 친화적 — stderr 분리

진행/요약 메시지는 stderr로, 결과는 stdout으로 분리됩니다. 그래서 다른 도구와 자연스럽게 조합 가능합니다.

```bash
# JSON을 jq로 후처리 (stderr는 화면에 그대로)
mantylog -f error --format json | jq '.line'

# 결과만 파일로 (요약은 화면에)
mantylog -f error --format json > errors.jsonl
```

## 프로젝트 구조

```
src/
├── main.rs    진입점 + 글루 코드
├── cli.rs     Args / LevelArg / OutputFormat (clap)
├── level.rs   LogLevel + From<LevelArg> + Display + parse_log_level
└── parse.rs   first_token
```

## 의존성

- [clap](https://crates.io/crates/clap) — CLI 인자 파싱
- [serde](https://crates.io/crates/serde) — 직렬화 인프라
- [serde_json](https://crates.io/crates/serde_json) — JSON 어댑터

## 테스트

```bash
cargo test
```

## 라이선스

학습용 프로젝트로 자유롭게 참고/사용하세요.
