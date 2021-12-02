data Command = Forward Int | Up Int | Down Int | Fail

toCommand :: (String, Int) -> Command
toCommand ("forward", n) = Forward n
toCommand ("up"     , n) = Up n
toCommand ("down"   , n) = Down n
toCommand (_        , _) = Fail

convert :: [String] -> (String, Int)
convert ss = (ss !! 0, read (ss !! 1) :: Int)

type Aim = Int
type Horizontal = Int
type Depth = Int

horizontal :: (Horizontal, Depth, Aim) -> Horizontal
horizontal (h, _, _) = h

depth :: (Horizontal, Depth, Aim) -> Depth
depth (_, d, _) = d

aim :: (Horizontal, Depth, Aim) -> Aim
aim (_, _, a) = a

updateState :: Command -> (Horizontal, Depth, Aim) -> (Horizontal, Depth, Aim)
updateState (Forward n) (h, d, a) = (h + n, d + (a * n), a)
updateState (Up n)      (h, d, a) = (h, d, (a - n))
updateState (Down n)    (h, d, a) = (h, d, (a + n))
updateState _           (h, d, a) = (h, d, a)

compute :: (Horizontal, Depth, Aim) -> Int
compute (h, d, _) = h * d

execute :: [Command] -> (Horizontal, Depth, Aim)
execute cmds = go cmds (0, 0, 0)
  where go []     state = state
        go (x:xs) state = go xs (updateState x state)

solve :: [Command] -> Int
solve cmds = compute $ execute cmds

main :: IO ()
main = interact $ show . solve . map toCommand . map convert . map words . lines
