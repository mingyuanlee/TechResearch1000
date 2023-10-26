package main

import (
	"crypto/sha256"
	"math"
)

type MerkleTree struct {
	rows []Row
}

type Row [][32]byte

// Sum256 returns the SHA256 checksum of the data.
func DoubleHash(input []byte) [32]byte {
	firstHashing := sha256.Sum256(input)
	return sha256.Sum256(firstHashing[:])
}

func JoinAndHash(left [32]byte, right [32]byte) [32]byte {
	combined := left[:]
	combined = append(combined, right[:]...)
	return DoubleHash(combined)
}

// the new row length is ceil(rowBeneath.length / 2)
// if the length of the row is add, then the single element at the will be copied twice
func makeRowAbove(below Row) Row {
	size := int(math.Ceil(float64(len(below)) / 2.0))
	row := make([][32]byte, size)
	for i, _ := range row {
		leftChild := i * 2
		rightChild := leftChild + 1
		if rightChild <= len(below)-1 {
			row[i] = JoinAndHash(below[leftChild], below[rightChild])
		} else {
			row[i] = JoinAndHash(below[leftChild], below[leftChild])
		}
	}
	return row
}

func NewMerkleTree(bottomRow Row) (tree MerkleTree) {
	tree.rows = append(tree.rows, bottomRow)
	rowBeneath := bottomRow
	for {
		rowAbove := makeRowAbove(rowBeneath)
		tree.rows = append(tree.rows, rowAbove)
		rowBeneath = rowAbove
		if tree.isComplete() {
			break
		}
	}
	return
}

func (tree MerkleTree) topRow() Row {
	return tree.rows[len(tree.rows)-1]
}

func (tree MerkleTree) isComplete() bool {
	return len(tree.topRow()) == 1
}

func (tree MerkleTree) MerkleRoot() [32]byte {
	return tree.topRow()[0]
}

// search a tree: Proof of inclusion
