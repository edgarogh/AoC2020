chunker = fn
  "\n", acc -> {:cont, acc, []}
  line, acc -> {:cont, [(line |> String.trim |> String.split) | acc]} # reversed but doesn't matter much
end

# Validation

in_range = fn n, lower, upper ->
  n = if is_binary(n) do
    {n, _} = Integer.parse n
    n
  else
    n
  end
  (n >= lower) and (n <= upper)
end

validate = fn
  ["byr", year] -> in_range.(year, 1920, 2002)
  ["iyr", year] -> in_range.(year, 2010, 2020)
  ["eyr", year] -> in_range.(year, 2020, 2030)
  ["hgt", height] -> case Integer.parse(height, 10) do
    {height, "cm"} -> in_range.(height, 150, 193)
    {height, "in"} -> in_range.(height, 59, 76)
    _ -> false
  end
  ["hcl", "#" <> hex] -> String.match?(hex, ~r/^[0-9a-f]{6}$/)
  ["ecl", color] -> Enum.member?(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"], color)
  ["pid", pid] -> String.match?(pid, ~r/^[0-9]{9}$/)
  _ -> false
end

# Input processing 1

flatten_passport_keys = fn list ->
  list
    |> List.flatten
    |> Enum.map(&(String.split(&1, ":") |> Enum.at(0)))
end

count_fields = fn passport ->
  passport |> Enum.filter(&(&1 != "cid")) |> Enum.count
end

# Entry point 1

IO.inspect (
  File.stream!("input.txt")
    |> Stream.chunk_while([], chunker, &({:cont, &1, []}))
    |> Stream.map(flatten_passport_keys)
    |> Stream.map(count_fields)
    |> Stream.filter(&(&1 == 7))
    |> Enum.count
)

# Input processing 2

process_passport = fn list ->
  list
    |> List.flatten
    |> Enum.map(&(validate.(String.split(&1, ":"))))
    |> Enum.filter(&(&1))
    |> Enum.count
end

# Entry point 2

IO.inspect (
  File.stream!("input.txt")
    |> Stream.chunk_while([], chunker, &({:cont, &1, []}))
    |> Stream.map(process_passport)
    |> Stream.filter(&(&1 == 7))
    |> Enum.count
)
