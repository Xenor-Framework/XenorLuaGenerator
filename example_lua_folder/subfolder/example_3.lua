--@ desc A simple greeting function that demonstrates
--@ desc multi-line descriptions across multiple
--@ desc comment lines
--@ param name string The person's name
--@ param greeting string Optional greeting word
function greet(name, greeting)
    greeting = greeting or "Hello"
    return greeting .. ", " .. name .. "!"
end