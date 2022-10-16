module Main where
  data TreeNode 
    = TreeNode 
    { val :: Int 
    , left :: TreeNode
    , right :: TreeNode
    }

  data Target 
    = AnyOf Int Int
    | One TreeNode Int
    | Done TreeNode

  lowestCommonAncestor :: TreeNode -> TreeNode -> TreeNode -> TreeNode
  lowestCommonAncestor root p q = lowestCommonAncestor_ (AnyOf $ val p $ val q) root
    where
      lowestCommonAncestor_ :: Target -> TreeNode -> TreeNode
      lowestCommonAncestor_ (One node a) (TreeNode val left right)
        = if val node == a 
            then Done node
            else case lowestCommonAncestor_ left target of
              Done _ -> Done node
              _ -> lowestCommonAncestor_ right target

      lowestCommonAncestor_ node (AnyOf (a, b)) 
        = if val node == a 
            then case lowestCommonAncestor_ (left node) (One b) of 
              Done _ -> Done node
              _ -> case lowestCommonAncestor_ (right node) of
                Done _ -> Done node
                _ -> One b

  main :: IO ()
  main = do
    putStrLn "hello world"
