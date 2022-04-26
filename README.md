# The json_static project

If, for our program, we are sure that we know the structure of the JSON file, a statically typed technique can and should be used instead. It is shown in the json_static project. The situation here is similar to that of the projects processing the TOML file.
The source code of the static version first declares three structs—one for every object type contained in the JSON file we are going to process. Each struct is preceded by the following attribute:
```
  #[derive(Deserialize, Serialize, Debug)]
```
  
### Storing and Retrieving Data

Let's understand the preceding snippet:
1. The Deserialize trait is required to parse (that is, read) JSON strings into this struct.
2. The Serialize trait is required to format (that is, write) this struct into a JSON string.
3. The Debug trait is just handy for printing this struct on a debug trace.
The JSON string is parsed using
the `serde_json::from_str::<SalesAndProducts>` function. Then, the code to increment the quantity of sold oranges becomes quite simple:
```
   sales_and_products.sales[1].quantity += 1.5
```
