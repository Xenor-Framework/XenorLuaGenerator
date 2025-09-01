-- @class Math
-- @desc My Add function is very cool!
-- @param x, number, Value 1
-- @param y, number, Value 2
-- @return Sum of two numbers
function Add(x,y)
    return x+y
end

--@ class Math
--@ desc Subtracts the second number from the first
--@ param x: number The first number
--@ param y: number The second number to subtract
--@ return number The result of x - y
function Math.subtract(x, y)
    return x - y
end

-- a simple comment

--@ class Array
--@ desc Finds the maximum value in an array of numbers
--@ param arr table An array of numbers
--@ return number The maximum value found
--@ return boolean True if array was not empty, false otherwise
function Array.max(arr)
    if(#arr == 0) then
        return nil, false
    end
    
    local max_val = arr[1]
    for(i = 2, #arr) do
        if arr[i] > max_val then
            max_val = arr[i]
        end
    end
    
    return max_val, true
end