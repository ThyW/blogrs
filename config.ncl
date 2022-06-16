let Address = (fun label value =>
  let regexpr = m#"^http(s?)://((\w+\.)?\w+\.\w+|((2[0-5]{2}|1[0-9]{2}|[0-9]{1,2})\.){3}(2[0-5]{2}|1[0-9]{2}|[0-9]{1,2}))(/)?$"#m in
  if builtins.is_str value && 
  strings.is_match regexpr value then
    value
  else
    contracts.blame_with "must be a valid ip address" label) in

let Port = contracts.from_predicate (fun value =>
  builtins.is_num value &&
  value <= 65500 &&
  value > 0) in

let WorkerCount = contracts.from_predicate (fun value =>
  builtins.is_num value &&
  value % 2 == 0) in

let KeepAlive = contracts.from_predicate (fun value =>
  builtins.is_num value) in

let ReadTimeout = contracts.from_predicate (fun value =>
  builtins.is_num value) in

let WriteTimeout = contracts.from_predicate (fun value =>
  builtins.is_num value) in

let LogLevel = (fun label value =>
  let available = ["critical", "normal", "debug", "off"] in
  if lists.any (fun x => x == strings.substring 0 (strings.length value) value ) available then
    value
  else
    contracts.blame_with "Must be one of critical, normal, debug or off" label ) in

let SecretKey = contracts.from_predicate (fun value =>
  builtins.is_str value) in

let Limit = contracts.from_predicate (fun value =>
  builtins.is_num value) in

// TODO
let Scheme = {
  address | Address
          | doc "Ip address or domain name to deploy on."
          | default = "http://localhost.com",
  port | Port
       | doc "Port to deploy on."
       | default = 8000,
  worker_count | WorkerCount
               | doc "Number of workers: number of cpus * 2"
               | default = 4,
  keep_alive | KeepAlive
             | doc "keepalive"
             | default = 10,
  read_timeout | ReadTimeout
               | doc "read timeout"
               | default = 5,
  write_timeout | WriteTimeout
                | doc "write timeout"
                | default = 10,
  log | LogLevel
      | default = "normal",
  secret_key  | SecretKey
              | default = "",
// it would be better to make this a concrete contract for better checking and defaults
  limits | { _ : Limit }
         | default = 
    { json =  nums.pow 1024 2,
      forms = 32 * 1024,
      file = nums.pow 1024 2,
      string = 8 * 1024,
      bytes = 8 * 1024,
      msgpack = nums.pow 1024 2
    },
} in

let Config = {
  development | Scheme
              | default = {},
  staging | Scheme
          | default = {},
  production | Scheme
             | default = {}
} in

let dev_config = {
  address = "https://google.com",
  port = 880,
  worker_count = 20,
  keep_alive = 10,
  read_timeout = 5,
  write_timeout = 5,
  log = "off",
  secret_key = "my super secret key",
  limits = {}
} in

let stg_config = {
  address = "https://something.com",
} in

let prod_config = {
  port = 53,
  limits = {
    json = 5,
    msgpack = 20,
  }
} in

let full = {
  development = dev_config,
  staging = stg_config,
  production = prod_config
} in

// let out = full.development.limits & {json = 5} in

full | Config
