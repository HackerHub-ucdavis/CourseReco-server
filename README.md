# CourseReco-server

API server for the Course Recommender project 

## Usage

```bash
cargo run
```

## API

Currently only one service is supported 

```bash
> curl -X GET http://localhost:8000/recom -d '{"liked": "MAT 135A", "k": 3, "subjects": "MAT,ECS,STA" }'
STA 131A,STA 103,ECS 020
```

# Acknowledgement

Template of this server is from [zupzup/rust-minimal-web-service-hyper](https://github.com/zupzup/rust-minimal-web-service-hyper).
Visit [this blog](https://blog.logrocket.com/a-minimal-web-service-in-rust-using-hyper/) for detail explanation.

# ToDo
* [ ] Support for other API calls
* [ ] log system instead of printing everything
* [ ] Integration with [SchedGo](https://join.schedgo.com/docs/intro/), respond with JSON of all course info from SchedGo
* [ ] better json response format
* [ ] better error handling system (no panic)