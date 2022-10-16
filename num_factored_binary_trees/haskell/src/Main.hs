module Main where
  import qualified Data.Map as Map

  data Leafs
    = Single Int
    | Pair Node Node

  data Node 
    = Node 
    { val :: Int
    , leafs :: [Leafs]
    }

  numFactoredBinaryTrees :: [Int] -> [Node]
  numFactoredBinaryTrees ns = process Map.empty
    where
      process :: [Int] -> 

  main :: IO ()
  main = do
    putStrLn "hello world"
