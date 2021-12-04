--import Data.Char
--import Data.List
--import Data.Function
import Data.List.Split (chunksOf)
import qualified Data.Text as T

type Draw  = [Int]
type Board = [[Int]]

-- First line in input contains a command separated list of numbers
-- drawn. That is followed by an empty line and a series of 5x5 grids of
-- numbers where numbers are separated by spaces and grids are separated
-- by empty lines.

readDrawNumbers :: String -> [Int]
readDrawNumbers input = map read $ map T.unpack $ T.splitOn (T.pack ",") (T.pack input)

createBoard :: [Int] -> Board
createBoard = chunksOf 5

createBoards :: [[Int]] -> [Board]
createBoards = map createBoard

markNumberInBoard :: Int -> Board -> Board
markNumberInBoard num board = createBoard $ map (1+) $ concat board

markNumber :: Int -> [Board] -> [Board]
markNumber num boards = map (markNumberInBoard num) boards

markNumbers :: [Int] -> [Board] -> [Board]
markNumbers nums boards = fmap markNumber nums >> boards

solve :: [String] -> Int
solve = undefined

main :: IO ()
main = do
  line <- getLine
  let drawn = readDrawNumbers line
  putStrLn $ unwords $ map show drawn

  -- Consume empty line
  _ <- getLine

  -- Get all boards
  contents <- getContents
  let input = lines contents
  let board_data = chunksOf 5 input
  let boards = createBoards $ (map . map) read board_data

  return ()
