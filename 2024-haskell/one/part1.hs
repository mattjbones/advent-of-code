import System.IO  
import Control.Monad
import Data.List.Split

-- main = do  
--     let list = []
--     handle <- openFile "sample" ReadMode
--     contents <- hGetContents handle
--     let singlewords = contents
--         list ++ singlewords
--     print list
--     hClose handle   

main = do  
    contents <- readFile "sample"
    -- print . map readInt . words $ contents
    let list = lines $ contents
    -- print list
    -- print . map splitter list 
    print . map splitter list

    -- print . lines $ contents

-- alternately, main = print . map readInt . words =<< readFile "test.txt"


splitter :: String -> [String]
splitter line = splitOn "   " line

getLines :: String -> [String]
getLines file = lines file

readInt :: String -> Int
readInt = read


f :: [String] -> [Int]
f = map read
