pub fn next(mut secret_number: usize) -> usize {
  secret_number = (secret_number ^ (secret_number * 64)) % 16777216;
  secret_number = (secret_number ^ (secret_number / 32)) % 16777216;
  (secret_number ^ (secret_number * 2048)) % 16777216
}
