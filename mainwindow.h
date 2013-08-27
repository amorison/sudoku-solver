#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QWidget>
#include <QMainWindow>
#include <QComboBox>
#include <QMessageBox>
#include <QAction>
#include <QString>
#include <QList>
#include <QTextStream>
#include <QFileDialog>
#include <QUrl>
#include <QDesktopServices>

namespace Ui {
class MainWindow;
}

class MainWindow : public QMainWindow
{
    Q_OBJECT
    
public:
    explicit MainWindow(QWidget *parent = 0);
    ~MainWindow();
    bool boardIsValid(); //check if there are no twice the same number in same area
    void backtracking(); //backtracking method used to solve any grid (exploration of the tree of solutions)
    void goBack(); //go to the previous node in the tree
    void writeInFile(); //write all the solutions of a grid in a txt file (asked after count if less than 1001 solutions)

public slots:
    void caseChanged(int index); //change during input in real time the boxes to avoid that same number are in same area
    void clearBoard();
    void count(); //display the first solution of a grid, count the number of solution and ask to write solutions in a file

private:
    Ui::MainWindow *ui;
    void initAtt();//initialisation of attributes
    bool noMoreSolution; //true when everything has been tested during backtrack
    int board[81];//linear representation of the plate
    int numPossible[81][9];//1st number : box, 2nd number : chiffer. 0 if the box can contains the chiffer.
    QComboBox *listBox[81];//list of combo box : make the link between the number of the box and the real combobox
    int boxesLinked[81][20];//the 20 boxes linked to the box 0-80 (in the same square, column and row)
    QList<int> voidBoxes; //list of unsolved boxes
    QList<int> backtrackList; //list of box where a modification have been made. When choice, 100 is added to the index.
    QIcon iconOk;
    QIcon iconEr;

    /*Backtracking is used to solve any sudoku grid. board variable contains the chiffer. The backtracking method chooses
the box wich has the less possibilities in voidBoxes, and put the first chiffer that is possible. numPossible is updated
(using boxesLinked). When a box has no possibilities, this is because a previous choosen number is false. The goBack method
remove boxes where a number has been put (using the backtrackList), and if more than one number were possible in a box, the
next one is choosed.*/
};

#endif // MAINWINDOW_H
