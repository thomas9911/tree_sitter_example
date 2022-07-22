json = """

{
    "data": true,
    "list": [1,2,3,4]
}


"""

TreeSitterExample.stuff(json) |> IO.inspect()
