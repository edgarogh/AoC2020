function find(input)
    prev = {}

    for line in input do
        num = tonumber(line)
        
        for _, nump in ipairs(prev) do
            if 2020 == (nump + num) then
                return nump * num
            end
        end

        table.insert(prev, num)
    end
end

print(find(io.lines("input.txt")))

function find2(input)
    prev = {}

    for line in input do
        num = tonumber(line)
        
        for _, nump in ipairs(prev) do
            for _, nump2 in ipairs(prev) do
                if 2020 == (nump + nump2 + num) then
                    return nump * nump2 * num
                end
            end
        end

        table.insert(prev, num)
    end
end

print(find2(io.lines("input.txt")))

