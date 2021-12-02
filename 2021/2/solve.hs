data Command = Forward Int | Up Int | Down Int | Fail

toCommand :: (String, Int) -> Command
toCommand ("forward", n) = Forward n
toCommand ("up"     , n) = Up n
toCommand ("down"   , n) = Down n
toCommand (_        , _) = Fail

convert :: [String] -> (String, Int)
convert ss = (ss !! 0, read (ss !! 1) :: Int)

updateState :: Command -> (Int, Int) -> (Int, Int)
updateState (Forward n) (x, y) = (x + n, y)
updateState (Up n)      (x, y) = (x, y - n)
updateState (Down n)    (x, y) = (x, y + n)
updateState _           (x, y) = (x, y)

compute :: [(Int, Int)] -> Int
compute states = x * y
  where fsts = map fst states
        snds = map snd states
        x = sum fsts
        y = sum snds

execute :: [Command] -> Int
execute cmds = compute $ map (\cmd -> updateState cmd (0, 0)) cmds

solve :: [Command] -> Int
solve cmds = execute cmds

main :: IO ()
main = interact $ show . solve . map toCommand . map convert . map words . lines
