#include "mainwindow.h"
#include "ui_mainwindow.h"

MainWindow::MainWindow(QWidget *parent) :
    QMainWindow(parent),
    ui(new Ui::MainWindow)
{
    ui->setupUi(this);
    initAtt();
    QObject::connect(ui->butClear,SIGNAL(clicked()),this,SLOT(clearBoard()));
    QObject::connect(ui->butSolve,SIGNAL(clicked()),this,SLOT(count()));
    for (int i=0;i<81;i++)
        QObject::connect(listBox[i],SIGNAL(activated(int)),this,SLOT(caseChanged(int)));
}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::caseChanged(int index)
{
    int box(0), boxTmp(0);
    ui->numOfSol->clear();
    for (int i=0;i<81;i++)
    {
        if (sender()==listBox[i])
        {
            box=i;
            break;
        }
    }
    listBox[box]->setItemIcon(board[box],QIcon());
    if (board[box]!=0)//other number were choosed before, upload numPossible to reput that possible in the linked boxes
    {
        for (int i=0;i<20;i++)
        {
            boxTmp=boxesLinked[box][i];
            numPossible[boxTmp][board[box]-1]--;
            if (numPossible[boxTmp][board[box]-1]==0 && board[box]==listBox[boxTmp]->currentIndex())
                listBox[boxTmp]->setItemIcon(board[boxTmp],iconOk);
        }
    }
    if (index!=0)//a new number is choosed, upload numPossible to put that impossible in the linked boxes
    {
        listBox[box]->setItemIcon(index,iconOk);
        for (int i=0;i<20;i++)
        {
            boxTmp=boxesLinked[box][i];
            numPossible[boxTmp][index-1]++;
            if (index==listBox[boxTmp]->currentIndex())
            {
                listBox[box]->setItemIcon(index,iconEr);
                listBox[boxTmp]->setItemIcon(index,iconEr);
                ui->butSolve->setEnabled(false);
            }
        }
    }
    board[box]=index;
    if (boardIsValid())
        ui->butSolve->setEnabled(true);
}

void MainWindow::clearBoard()
{
    for (int i=0;i<81;i++)
    {
        listBox[i]->setItemIcon(board[i],QIcon());
        listBox[i]->setCurrentIndex(0);
        board[i]=0;
        for (int j=0;j<9;j++)
            numPossible[i][j]=0;
    }
    ui->butSolve->setEnabled(true);
    ui->numOfSol->clear();
}

bool MainWindow::boardIsValid()
{
    for (int i=0;i<81;i++)
    {
        if (board[i]!=0)
        {
            if (numPossible[i][board[i]-1]!=0)
                return false;
        }
    }
    return true;
}

void MainWindow::backtracking()
{
    int box(0), num(0), possible(0), boxTmp(0), numTmp(0), posTmp(0); //box : la box à changer ; num : le numéro à mettre ; possible : le nombre de chiffres qu'on peut mettre ; boxTmp : la box étudiée au tour de boucle
    while (!voidBoxes.isEmpty())
    {
        box=0;
        num=0;
        possible=10;
        for (int i=0;i<voidBoxes.size();i++) //find the box wich has the less possibilities
        {
            boxTmp=voidBoxes.at(i);
            numTmp=0;
            posTmp=0;
            for (int j=0;j<9;j++)
            {
                if (numPossible[boxTmp][j]==0)
                {
                    posTmp++;
                    if (posTmp==1)
                        numTmp=j+1;
                }
            }
            if (posTmp<possible)
            {
                box=boxTmp;
                num=numTmp;
                possible=posTmp;
            }
            if (possible==0 || possible==1)
                break;
        }
        if (possible==0) //no possibilities in one box->bad choice somewhere
            goBack();
        else //put the first possible number and update the backtrack queue
        {
            board[box]=num;
            for (int i=0;i<20;i++)
                numPossible[boxesLinked[box][i]][num-1]++;
            voidBoxes.removeOne(box);
            if (possible>1)
                box+=100;
            backtrackList.append(box);
        }
    }
}

void MainWindow::goBack()
{
    bool ok(false);
    int box(0), boxTmp(0), num(0);
    while (!ok) //while the previous node or the root of the tree is not reached
    {
        if (backtrackList.isEmpty()) //root is reached
        {
            if (!voidBoxes.isEmpty())
            {
                voidBoxes.clear();
                noMoreSolution=true;
            }
            ok=true;
        }
        else //go back up to the previous node
        {
            box=backtrackList.takeLast();
            boxTmp=box;
            if (boxTmp>=100)
                boxTmp-=100;
            num=board[boxTmp];
            for (int i=0;i<20;i++)
                numPossible[boxesLinked[boxTmp][i]][num-1]--;
            board[boxTmp]=0;
            voidBoxes.prepend(boxTmp);
            if (box>=100) //more than one chiffer where possible (so this is a node), take the next possible chiffer
            {
                for (int i=num;i<9;i++)
                {
                    if (numPossible[boxTmp][i]==0)
                    {
                        board[boxTmp]=i+1;
                        for (int j=0;j<20;j++)
                            numPossible[boxesLinked[boxTmp][j]][i]++;
                        voidBoxes.removeFirst();
                        backtrackList.append(box);
                        ok=true;
                        break;
                    }
                }
            }
        }
    }
}

void MainWindow::count()
{
    noMoreSolution=false;
    voidBoxes.clear();
    backtrackList.clear();
    for (int i=0;i<81;i++)
    {
        if (board[i]==0)
            voidBoxes.append(i);
    }
    int numOfSol=0;
    do
    {
        backtracking();
        if (!noMoreSolution)
        {
            numOfSol+=1;
            if (numOfSol==1)
            {
                for (int i=0;i<81;i++)
                    listBox[i]->setCurrentIndex(board[i]);
            }
            if (numOfSol==1001)
                break;
            goBack();
        }
    }while (!noMoreSolution); //explore completely the tree of solutions (stop if more than 1000 solutions are found)
    QString msg;
    msg.setNum(numOfSol);
    switch (numOfSol)
    {
    case 0:
        msg="No solution";
        break;
    case 1:
        msg="One solution";
        break;
    case 1001:
        msg="More than 1000 solutions";
        break;
    default:
        msg+=" solutions";
        break;
    }
    ui->numOfSol->setText(msg);
    if (numOfSol>1 && numOfSol<1001 && QMessageBox::question(this,"Sudoku solver","They are "+msg+".\nDo you want to write them in a file ?")==QMessageBox::Yes)
        writeInFile();
}

void MainWindow::writeInFile()
{
    QString path= QFileDialog::getSaveFileName(this,"Choose a file",QDir::homePath(),"Text file (*.txt)");
    QFile file(path);
    if (path=="")
        return;
    if (!file.open(QIODevice::WriteOnly|QIODevice::Text))
    {
        QMessageBox::critical(this,"Sudoku solver","Impossible to open "+path+".");
        return;
    }
    QTextStream flux(&file);
    flux << "Sudoku solver by @dri1\n\nSolutions of :\n";
    noMoreSolution=false;
    voidBoxes.clear();
    backtrackList.clear();
    for (int i=0;i<81;i++)
    {
        if (board[i]==0)
        {
            voidBoxes.append(i);
            flux << " ";
        }
        else
            flux << board[i];
        if ((i+1)%27==0 && i!=80)
        {
            flux << "\n";
            for (int j=0;j<11;j++)
                flux << "-";
        }
        if ((i+1)%9==0)
            flux << "\n";
        else if ((i+1)%3==0)
            flux << "|";
    }
    flux << "\n";
    do
    {
        backtracking();
        if (!noMoreSolution)
        {
            flux << "\n\n\n";
            for (int i=0;i<81;i++)
            {
                flux << board[i];
                if ((i+1)%27==0 && i!=80)
                {
                    flux << "\n";
                    for (int j=0;j<11;j++)
                        flux << "-";
                }
                if ((i+1)%9==0)
                    flux << "\n";
                else if ((i+1)%3==0)
                    flux << "|";
            }
            goBack();
        }
    }while (!noMoreSolution); //same exploration than in count, but write solutions in a file
    QDesktopServices::openUrl(QUrl(path));
}

void MainWindow::initAtt()
{
    for (int i=0;i<81;i++)
    {
        board[i]=0;
        for (int j=0;j<9;j++)
            numPossible[i][j]=0;
    }

    QPixmap pixOk(2,10), pixEr(2,10);
    pixOk.fill(Qt::blue);
    pixEr.fill(Qt::red);
    iconOk=QIcon(pixOk);
    iconEr=QIcon(pixEr);

    boxesLinked[0][0]=1;boxesLinked[0][1]=2;boxesLinked[0][2]=9;boxesLinked[0][3]=10;boxesLinked[0][4]=11;boxesLinked[0][5]=18;boxesLinked[0][6]=19;boxesLinked[0][7]=20;boxesLinked[0][8]=3;boxesLinked[0][9]=4;boxesLinked[0][10]=5;boxesLinked[0][11]=6;boxesLinked[0][12]=7;boxesLinked[0][13]=8;boxesLinked[0][14]=27;boxesLinked[0][15]=36;boxesLinked[0][16]=45;boxesLinked[0][17]=54;boxesLinked[0][18]=63;boxesLinked[0][19]=72;
    boxesLinked[1][0]=0;boxesLinked[1][1]=2;boxesLinked[1][2]=9;boxesLinked[1][3]=10;boxesLinked[1][4]=11;boxesLinked[1][5]=18;boxesLinked[1][6]=19;boxesLinked[1][7]=20;boxesLinked[1][8]=3;boxesLinked[1][9]=4;boxesLinked[1][10]=5;boxesLinked[1][11]=6;boxesLinked[1][12]=7;boxesLinked[1][13]=8;boxesLinked[1][14]=28;boxesLinked[1][15]=37;boxesLinked[1][16]=46;boxesLinked[1][17]=55;boxesLinked[1][18]=64;boxesLinked[1][19]=73;
    boxesLinked[2][0]=0;boxesLinked[2][1]=1;boxesLinked[2][2]=9;boxesLinked[2][3]=10;boxesLinked[2][4]=11;boxesLinked[2][5]=18;boxesLinked[2][6]=19;boxesLinked[2][7]=20;boxesLinked[2][8]=3;boxesLinked[2][9]=4;boxesLinked[2][10]=5;boxesLinked[2][11]=6;boxesLinked[2][12]=7;boxesLinked[2][13]=8;boxesLinked[2][14]=29;boxesLinked[2][15]=38;boxesLinked[2][16]=47;boxesLinked[2][17]=56;boxesLinked[2][18]=65;boxesLinked[2][19]=74;
    boxesLinked[3][0]=4;boxesLinked[3][1]=5;boxesLinked[3][2]=12;boxesLinked[3][3]=13;boxesLinked[3][4]=14;boxesLinked[3][5]=21;boxesLinked[3][6]=22;boxesLinked[3][7]=23;boxesLinked[3][8]=0;boxesLinked[3][9]=1;boxesLinked[3][10]=2;boxesLinked[3][11]=6;boxesLinked[3][12]=7;boxesLinked[3][13]=8;boxesLinked[3][14]=30;boxesLinked[3][15]=39;boxesLinked[3][16]=48;boxesLinked[3][17]=57;boxesLinked[3][18]=66;boxesLinked[3][19]=75;
    boxesLinked[4][0]=3;boxesLinked[4][1]=5;boxesLinked[4][2]=12;boxesLinked[4][3]=13;boxesLinked[4][4]=14;boxesLinked[4][5]=21;boxesLinked[4][6]=22;boxesLinked[4][7]=23;boxesLinked[4][8]=0;boxesLinked[4][9]=1;boxesLinked[4][10]=2;boxesLinked[4][11]=6;boxesLinked[4][12]=7;boxesLinked[4][13]=8;boxesLinked[4][14]=31;boxesLinked[4][15]=40;boxesLinked[4][16]=49;boxesLinked[4][17]=58;boxesLinked[4][18]=67;boxesLinked[4][19]=76;
    boxesLinked[5][0]=3;boxesLinked[5][1]=4;boxesLinked[5][2]=12;boxesLinked[5][3]=13;boxesLinked[5][4]=14;boxesLinked[5][5]=21;boxesLinked[5][6]=22;boxesLinked[5][7]=23;boxesLinked[5][8]=0;boxesLinked[5][9]=1;boxesLinked[5][10]=2;boxesLinked[5][11]=6;boxesLinked[5][12]=7;boxesLinked[5][13]=8;boxesLinked[5][14]=32;boxesLinked[5][15]=41;boxesLinked[5][16]=50;boxesLinked[5][17]=59;boxesLinked[5][18]=68;boxesLinked[5][19]=77;
    boxesLinked[6][0]=7;boxesLinked[6][1]=8;boxesLinked[6][2]=15;boxesLinked[6][3]=16;boxesLinked[6][4]=17;boxesLinked[6][5]=24;boxesLinked[6][6]=25;boxesLinked[6][7]=26;boxesLinked[6][8]=0;boxesLinked[6][9]=1;boxesLinked[6][10]=2;boxesLinked[6][11]=3;boxesLinked[6][12]=4;boxesLinked[6][13]=5;boxesLinked[6][14]=33;boxesLinked[6][15]=42;boxesLinked[6][16]=51;boxesLinked[6][17]=60;boxesLinked[6][18]=69;boxesLinked[6][19]=78;
    boxesLinked[7][0]=6;boxesLinked[7][1]=8;boxesLinked[7][2]=15;boxesLinked[7][3]=16;boxesLinked[7][4]=17;boxesLinked[7][5]=24;boxesLinked[7][6]=25;boxesLinked[7][7]=26;boxesLinked[7][8]=0;boxesLinked[7][9]=1;boxesLinked[7][10]=2;boxesLinked[7][11]=3;boxesLinked[7][12]=4;boxesLinked[7][13]=5;boxesLinked[7][14]=34;boxesLinked[7][15]=43;boxesLinked[7][16]=52;boxesLinked[7][17]=61;boxesLinked[7][18]=70;boxesLinked[7][19]=79;
    boxesLinked[8][0]=6;boxesLinked[8][1]=7;boxesLinked[8][2]=15;boxesLinked[8][3]=16;boxesLinked[8][4]=17;boxesLinked[8][5]=24;boxesLinked[8][6]=25;boxesLinked[8][7]=26;boxesLinked[8][8]=0;boxesLinked[8][9]=1;boxesLinked[8][10]=2;boxesLinked[8][11]=3;boxesLinked[8][12]=4;boxesLinked[8][13]=5;boxesLinked[8][14]=35;boxesLinked[8][15]=44;boxesLinked[8][16]=53;boxesLinked[8][17]=62;boxesLinked[8][18]=71;boxesLinked[8][19]=80;
    boxesLinked[9][0]=0;boxesLinked[9][1]=1;boxesLinked[9][2]=2;boxesLinked[9][3]=10;boxesLinked[9][4]=11;boxesLinked[9][5]=18;boxesLinked[9][6]=19;boxesLinked[9][7]=20;boxesLinked[9][8]=12;boxesLinked[9][9]=13;boxesLinked[9][10]=14;boxesLinked[9][11]=15;boxesLinked[9][12]=16;boxesLinked[9][13]=17;boxesLinked[9][14]=27;boxesLinked[9][15]=36;boxesLinked[9][16]=45;boxesLinked[9][17]=54;boxesLinked[9][18]=63;boxesLinked[9][19]=72;
    boxesLinked[10][0]=0;boxesLinked[10][1]=1;boxesLinked[10][2]=2;boxesLinked[10][3]=9;boxesLinked[10][4]=11;boxesLinked[10][5]=18;boxesLinked[10][6]=19;boxesLinked[10][7]=20;boxesLinked[10][8]=12;boxesLinked[10][9]=13;boxesLinked[10][10]=14;boxesLinked[10][11]=15;boxesLinked[10][12]=16;boxesLinked[10][13]=17;boxesLinked[10][14]=28;boxesLinked[10][15]=37;boxesLinked[10][16]=46;boxesLinked[10][17]=55;boxesLinked[10][18]=64;boxesLinked[10][19]=73;
    boxesLinked[11][0]=0;boxesLinked[11][1]=1;boxesLinked[11][2]=2;boxesLinked[11][3]=9;boxesLinked[11][4]=10;boxesLinked[11][5]=18;boxesLinked[11][6]=19;boxesLinked[11][7]=20;boxesLinked[11][8]=12;boxesLinked[11][9]=13;boxesLinked[11][10]=14;boxesLinked[11][11]=15;boxesLinked[11][12]=16;boxesLinked[11][13]=17;boxesLinked[11][14]=29;boxesLinked[11][15]=38;boxesLinked[11][16]=47;boxesLinked[11][17]=56;boxesLinked[11][18]=65;boxesLinked[11][19]=74;
    boxesLinked[12][0]=3;boxesLinked[12][1]=4;boxesLinked[12][2]=5;boxesLinked[12][3]=13;boxesLinked[12][4]=14;boxesLinked[12][5]=21;boxesLinked[12][6]=22;boxesLinked[12][7]=23;boxesLinked[12][8]=9;boxesLinked[12][9]=10;boxesLinked[12][10]=11;boxesLinked[12][11]=15;boxesLinked[12][12]=16;boxesLinked[12][13]=17;boxesLinked[12][14]=30;boxesLinked[12][15]=39;boxesLinked[12][16]=48;boxesLinked[12][17]=57;boxesLinked[12][18]=66;boxesLinked[12][19]=75;
    boxesLinked[13][0]=3;boxesLinked[13][1]=4;boxesLinked[13][2]=5;boxesLinked[13][3]=12;boxesLinked[13][4]=14;boxesLinked[13][5]=21;boxesLinked[13][6]=22;boxesLinked[13][7]=23;boxesLinked[13][8]=9;boxesLinked[13][9]=10;boxesLinked[13][10]=11;boxesLinked[13][11]=15;boxesLinked[13][12]=16;boxesLinked[13][13]=17;boxesLinked[13][14]=31;boxesLinked[13][15]=40;boxesLinked[13][16]=49;boxesLinked[13][17]=58;boxesLinked[13][18]=67;boxesLinked[13][19]=76;
    boxesLinked[14][0]=3;boxesLinked[14][1]=4;boxesLinked[14][2]=5;boxesLinked[14][3]=12;boxesLinked[14][4]=13;boxesLinked[14][5]=21;boxesLinked[14][6]=22;boxesLinked[14][7]=23;boxesLinked[14][8]=9;boxesLinked[14][9]=10;boxesLinked[14][10]=11;boxesLinked[14][11]=15;boxesLinked[14][12]=16;boxesLinked[14][13]=17;boxesLinked[14][14]=32;boxesLinked[14][15]=41;boxesLinked[14][16]=50;boxesLinked[14][17]=59;boxesLinked[14][18]=68;boxesLinked[14][19]=77;
    boxesLinked[15][0]=6;boxesLinked[15][1]=7;boxesLinked[15][2]=8;boxesLinked[15][3]=16;boxesLinked[15][4]=17;boxesLinked[15][5]=24;boxesLinked[15][6]=25;boxesLinked[15][7]=26;boxesLinked[15][8]=9;boxesLinked[15][9]=10;boxesLinked[15][10]=11;boxesLinked[15][11]=12;boxesLinked[15][12]=13;boxesLinked[15][13]=14;boxesLinked[15][14]=33;boxesLinked[15][15]=42;boxesLinked[15][16]=51;boxesLinked[15][17]=60;boxesLinked[15][18]=69;boxesLinked[15][19]=78;
    boxesLinked[16][0]=6;boxesLinked[16][1]=7;boxesLinked[16][2]=8;boxesLinked[16][3]=15;boxesLinked[16][4]=17;boxesLinked[16][5]=24;boxesLinked[16][6]=25;boxesLinked[16][7]=26;boxesLinked[16][8]=9;boxesLinked[16][9]=10;boxesLinked[16][10]=11;boxesLinked[16][11]=12;boxesLinked[16][12]=13;boxesLinked[16][13]=14;boxesLinked[16][14]=34;boxesLinked[16][15]=43;boxesLinked[16][16]=52;boxesLinked[16][17]=61;boxesLinked[16][18]=70;boxesLinked[16][19]=79;
    boxesLinked[17][0]=6;boxesLinked[17][1]=7;boxesLinked[17][2]=8;boxesLinked[17][3]=15;boxesLinked[17][4]=16;boxesLinked[17][5]=24;boxesLinked[17][6]=25;boxesLinked[17][7]=26;boxesLinked[17][8]=9;boxesLinked[17][9]=10;boxesLinked[17][10]=11;boxesLinked[17][11]=12;boxesLinked[17][12]=13;boxesLinked[17][13]=14;boxesLinked[17][14]=35;boxesLinked[17][15]=44;boxesLinked[17][16]=53;boxesLinked[17][17]=62;boxesLinked[17][18]=71;boxesLinked[17][19]=80;
    boxesLinked[18][0]=0;boxesLinked[18][1]=1;boxesLinked[18][2]=2;boxesLinked[18][3]=9;boxesLinked[18][4]=10;boxesLinked[18][5]=11;boxesLinked[18][6]=19;boxesLinked[18][7]=20;boxesLinked[18][8]=21;boxesLinked[18][9]=22;boxesLinked[18][10]=23;boxesLinked[18][11]=24;boxesLinked[18][12]=25;boxesLinked[18][13]=26;boxesLinked[18][14]=27;boxesLinked[18][15]=36;boxesLinked[18][16]=45;boxesLinked[18][17]=54;boxesLinked[18][18]=63;boxesLinked[18][19]=72;
    boxesLinked[19][0]=0;boxesLinked[19][1]=1;boxesLinked[19][2]=2;boxesLinked[19][3]=9;boxesLinked[19][4]=10;boxesLinked[19][5]=11;boxesLinked[19][6]=18;boxesLinked[19][7]=20;boxesLinked[19][8]=21;boxesLinked[19][9]=22;boxesLinked[19][10]=23;boxesLinked[19][11]=24;boxesLinked[19][12]=25;boxesLinked[19][13]=26;boxesLinked[19][14]=28;boxesLinked[19][15]=37;boxesLinked[19][16]=46;boxesLinked[19][17]=55;boxesLinked[19][18]=64;boxesLinked[19][19]=73;
    boxesLinked[20][0]=0;boxesLinked[20][1]=1;boxesLinked[20][2]=2;boxesLinked[20][3]=9;boxesLinked[20][4]=10;boxesLinked[20][5]=11;boxesLinked[20][6]=18;boxesLinked[20][7]=19;boxesLinked[20][8]=21;boxesLinked[20][9]=22;boxesLinked[20][10]=23;boxesLinked[20][11]=24;boxesLinked[20][12]=25;boxesLinked[20][13]=26;boxesLinked[20][14]=29;boxesLinked[20][15]=38;boxesLinked[20][16]=47;boxesLinked[20][17]=56;boxesLinked[20][18]=65;boxesLinked[20][19]=74;
    boxesLinked[21][0]=3;boxesLinked[21][1]=4;boxesLinked[21][2]=5;boxesLinked[21][3]=12;boxesLinked[21][4]=13;boxesLinked[21][5]=14;boxesLinked[21][6]=22;boxesLinked[21][7]=23;boxesLinked[21][8]=18;boxesLinked[21][9]=19;boxesLinked[21][10]=20;boxesLinked[21][11]=24;boxesLinked[21][12]=25;boxesLinked[21][13]=26;boxesLinked[21][14]=30;boxesLinked[21][15]=39;boxesLinked[21][16]=48;boxesLinked[21][17]=57;boxesLinked[21][18]=66;boxesLinked[21][19]=75;
    boxesLinked[22][0]=3;boxesLinked[22][1]=4;boxesLinked[22][2]=5;boxesLinked[22][3]=12;boxesLinked[22][4]=13;boxesLinked[22][5]=14;boxesLinked[22][6]=21;boxesLinked[22][7]=23;boxesLinked[22][8]=18;boxesLinked[22][9]=19;boxesLinked[22][10]=20;boxesLinked[22][11]=24;boxesLinked[22][12]=25;boxesLinked[22][13]=26;boxesLinked[22][14]=31;boxesLinked[22][15]=40;boxesLinked[22][16]=49;boxesLinked[22][17]=58;boxesLinked[22][18]=67;boxesLinked[22][19]=76;
    boxesLinked[23][0]=3;boxesLinked[23][1]=4;boxesLinked[23][2]=5;boxesLinked[23][3]=12;boxesLinked[23][4]=13;boxesLinked[23][5]=14;boxesLinked[23][6]=21;boxesLinked[23][7]=22;boxesLinked[23][8]=18;boxesLinked[23][9]=19;boxesLinked[23][10]=20;boxesLinked[23][11]=24;boxesLinked[23][12]=25;boxesLinked[23][13]=26;boxesLinked[23][14]=32;boxesLinked[23][15]=41;boxesLinked[23][16]=50;boxesLinked[23][17]=59;boxesLinked[23][18]=68;boxesLinked[23][19]=77;
    boxesLinked[24][0]=6;boxesLinked[24][1]=7;boxesLinked[24][2]=8;boxesLinked[24][3]=15;boxesLinked[24][4]=16;boxesLinked[24][5]=17;boxesLinked[24][6]=25;boxesLinked[24][7]=26;boxesLinked[24][8]=18;boxesLinked[24][9]=19;boxesLinked[24][10]=20;boxesLinked[24][11]=21;boxesLinked[24][12]=22;boxesLinked[24][13]=23;boxesLinked[24][14]=33;boxesLinked[24][15]=42;boxesLinked[24][16]=51;boxesLinked[24][17]=60;boxesLinked[24][18]=69;boxesLinked[24][19]=78;
    boxesLinked[25][0]=6;boxesLinked[25][1]=7;boxesLinked[25][2]=8;boxesLinked[25][3]=15;boxesLinked[25][4]=16;boxesLinked[25][5]=17;boxesLinked[25][6]=24;boxesLinked[25][7]=26;boxesLinked[25][8]=18;boxesLinked[25][9]=19;boxesLinked[25][10]=20;boxesLinked[25][11]=21;boxesLinked[25][12]=22;boxesLinked[25][13]=23;boxesLinked[25][14]=34;boxesLinked[25][15]=43;boxesLinked[25][16]=52;boxesLinked[25][17]=61;boxesLinked[25][18]=70;boxesLinked[25][19]=79;
    boxesLinked[26][0]=6;boxesLinked[26][1]=7;boxesLinked[26][2]=8;boxesLinked[26][3]=15;boxesLinked[26][4]=16;boxesLinked[26][5]=17;boxesLinked[26][6]=24;boxesLinked[26][7]=25;boxesLinked[26][8]=18;boxesLinked[26][9]=19;boxesLinked[26][10]=20;boxesLinked[26][11]=21;boxesLinked[26][12]=22;boxesLinked[26][13]=23;boxesLinked[26][14]=35;boxesLinked[26][15]=44;boxesLinked[26][16]=53;boxesLinked[26][17]=62;boxesLinked[26][18]=71;boxesLinked[26][19]=80;
    boxesLinked[27][0]=28;boxesLinked[27][1]=29;boxesLinked[27][2]=36;boxesLinked[27][3]=37;boxesLinked[27][4]=38;boxesLinked[27][5]=45;boxesLinked[27][6]=46;boxesLinked[27][7]=47;boxesLinked[27][8]=30;boxesLinked[27][9]=31;boxesLinked[27][10]=32;boxesLinked[27][11]=33;boxesLinked[27][12]=34;boxesLinked[27][13]=35;boxesLinked[27][14]=0;boxesLinked[27][15]=9;boxesLinked[27][16]=18;boxesLinked[27][17]=54;boxesLinked[27][18]=63;boxesLinked[27][19]=72;
    boxesLinked[28][0]=27;boxesLinked[28][1]=29;boxesLinked[28][2]=36;boxesLinked[28][3]=37;boxesLinked[28][4]=38;boxesLinked[28][5]=45;boxesLinked[28][6]=46;boxesLinked[28][7]=47;boxesLinked[28][8]=30;boxesLinked[28][9]=31;boxesLinked[28][10]=32;boxesLinked[28][11]=33;boxesLinked[28][12]=34;boxesLinked[28][13]=35;boxesLinked[28][14]=1;boxesLinked[28][15]=10;boxesLinked[28][16]=19;boxesLinked[28][17]=55;boxesLinked[28][18]=64;boxesLinked[28][19]=73;
    boxesLinked[29][0]=27;boxesLinked[29][1]=28;boxesLinked[29][2]=36;boxesLinked[29][3]=37;boxesLinked[29][4]=38;boxesLinked[29][5]=45;boxesLinked[29][6]=46;boxesLinked[29][7]=47;boxesLinked[29][8]=30;boxesLinked[29][9]=31;boxesLinked[29][10]=32;boxesLinked[29][11]=33;boxesLinked[29][12]=34;boxesLinked[29][13]=35;boxesLinked[29][14]=2;boxesLinked[29][15]=11;boxesLinked[29][16]=20;boxesLinked[29][17]=56;boxesLinked[29][18]=65;boxesLinked[29][19]=74;
    boxesLinked[30][0]=31;boxesLinked[30][1]=32;boxesLinked[30][2]=39;boxesLinked[30][3]=40;boxesLinked[30][4]=41;boxesLinked[30][5]=48;boxesLinked[30][6]=49;boxesLinked[30][7]=50;boxesLinked[30][8]=27;boxesLinked[30][9]=28;boxesLinked[30][10]=29;boxesLinked[30][11]=33;boxesLinked[30][12]=34;boxesLinked[30][13]=35;boxesLinked[30][14]=3;boxesLinked[30][15]=12;boxesLinked[30][16]=21;boxesLinked[30][17]=57;boxesLinked[30][18]=66;boxesLinked[30][19]=75;
    boxesLinked[31][0]=30;boxesLinked[31][1]=32;boxesLinked[31][2]=39;boxesLinked[31][3]=40;boxesLinked[31][4]=41;boxesLinked[31][5]=48;boxesLinked[31][6]=49;boxesLinked[31][7]=50;boxesLinked[31][8]=27;boxesLinked[31][9]=28;boxesLinked[31][10]=29;boxesLinked[31][11]=33;boxesLinked[31][12]=34;boxesLinked[31][13]=35;boxesLinked[31][14]=4;boxesLinked[31][15]=13;boxesLinked[31][16]=22;boxesLinked[31][17]=58;boxesLinked[31][18]=67;boxesLinked[31][19]=76;
    boxesLinked[32][0]=30;boxesLinked[32][1]=31;boxesLinked[32][2]=39;boxesLinked[32][3]=40;boxesLinked[32][4]=41;boxesLinked[32][5]=48;boxesLinked[32][6]=49;boxesLinked[32][7]=50;boxesLinked[32][8]=27;boxesLinked[32][9]=28;boxesLinked[32][10]=29;boxesLinked[32][11]=33;boxesLinked[32][12]=34;boxesLinked[32][13]=35;boxesLinked[32][14]=5;boxesLinked[32][15]=14;boxesLinked[32][16]=23;boxesLinked[32][17]=59;boxesLinked[32][18]=68;boxesLinked[32][19]=77;
    boxesLinked[33][0]=34;boxesLinked[33][1]=35;boxesLinked[33][2]=42;boxesLinked[33][3]=43;boxesLinked[33][4]=44;boxesLinked[33][5]=51;boxesLinked[33][6]=52;boxesLinked[33][7]=53;boxesLinked[33][8]=27;boxesLinked[33][9]=28;boxesLinked[33][10]=29;boxesLinked[33][11]=30;boxesLinked[33][12]=31;boxesLinked[33][13]=32;boxesLinked[33][14]=6;boxesLinked[33][15]=15;boxesLinked[33][16]=24;boxesLinked[33][17]=60;boxesLinked[33][18]=69;boxesLinked[33][19]=78;
    boxesLinked[34][0]=33;boxesLinked[34][1]=35;boxesLinked[34][2]=42;boxesLinked[34][3]=43;boxesLinked[34][4]=44;boxesLinked[34][5]=51;boxesLinked[34][6]=52;boxesLinked[34][7]=53;boxesLinked[34][8]=27;boxesLinked[34][9]=28;boxesLinked[34][10]=29;boxesLinked[34][11]=30;boxesLinked[34][12]=31;boxesLinked[34][13]=32;boxesLinked[34][14]=7;boxesLinked[34][15]=16;boxesLinked[34][16]=25;boxesLinked[34][17]=61;boxesLinked[34][18]=70;boxesLinked[34][19]=79;
    boxesLinked[35][0]=33;boxesLinked[35][1]=34;boxesLinked[35][2]=42;boxesLinked[35][3]=43;boxesLinked[35][4]=44;boxesLinked[35][5]=51;boxesLinked[35][6]=52;boxesLinked[35][7]=53;boxesLinked[35][8]=27;boxesLinked[35][9]=28;boxesLinked[35][10]=29;boxesLinked[35][11]=30;boxesLinked[35][12]=31;boxesLinked[35][13]=32;boxesLinked[35][14]=8;boxesLinked[35][15]=17;boxesLinked[35][16]=26;boxesLinked[35][17]=62;boxesLinked[35][18]=71;boxesLinked[35][19]=80;
    boxesLinked[36][0]=27;boxesLinked[36][1]=28;boxesLinked[36][2]=29;boxesLinked[36][3]=37;boxesLinked[36][4]=38;boxesLinked[36][5]=45;boxesLinked[36][6]=46;boxesLinked[36][7]=47;boxesLinked[36][8]=39;boxesLinked[36][9]=40;boxesLinked[36][10]=41;boxesLinked[36][11]=42;boxesLinked[36][12]=43;boxesLinked[36][13]=44;boxesLinked[36][14]=0;boxesLinked[36][15]=9;boxesLinked[36][16]=18;boxesLinked[36][17]=54;boxesLinked[36][18]=63;boxesLinked[36][19]=72;
    boxesLinked[37][0]=27;boxesLinked[37][1]=28;boxesLinked[37][2]=29;boxesLinked[37][3]=36;boxesLinked[37][4]=38;boxesLinked[37][5]=45;boxesLinked[37][6]=46;boxesLinked[37][7]=47;boxesLinked[37][8]=39;boxesLinked[37][9]=40;boxesLinked[37][10]=41;boxesLinked[37][11]=42;boxesLinked[37][12]=43;boxesLinked[37][13]=44;boxesLinked[37][14]=1;boxesLinked[37][15]=10;boxesLinked[37][16]=19;boxesLinked[37][17]=55;boxesLinked[37][18]=64;boxesLinked[37][19]=73;
    boxesLinked[38][0]=27;boxesLinked[38][1]=28;boxesLinked[38][2]=29;boxesLinked[38][3]=36;boxesLinked[38][4]=37;boxesLinked[38][5]=45;boxesLinked[38][6]=46;boxesLinked[38][7]=47;boxesLinked[38][8]=39;boxesLinked[38][9]=40;boxesLinked[38][10]=41;boxesLinked[38][11]=42;boxesLinked[38][12]=43;boxesLinked[38][13]=44;boxesLinked[38][14]=2;boxesLinked[38][15]=11;boxesLinked[38][16]=20;boxesLinked[38][17]=56;boxesLinked[38][18]=65;boxesLinked[38][19]=74;
    boxesLinked[39][0]=30;boxesLinked[39][1]=31;boxesLinked[39][2]=32;boxesLinked[39][3]=40;boxesLinked[39][4]=41;boxesLinked[39][5]=48;boxesLinked[39][6]=49;boxesLinked[39][7]=50;boxesLinked[39][8]=36;boxesLinked[39][9]=37;boxesLinked[39][10]=38;boxesLinked[39][11]=42;boxesLinked[39][12]=43;boxesLinked[39][13]=44;boxesLinked[39][14]=3;boxesLinked[39][15]=12;boxesLinked[39][16]=21;boxesLinked[39][17]=57;boxesLinked[39][18]=66;boxesLinked[39][19]=75;
    boxesLinked[40][0]=30;boxesLinked[40][1]=31;boxesLinked[40][2]=32;boxesLinked[40][3]=39;boxesLinked[40][4]=41;boxesLinked[40][5]=48;boxesLinked[40][6]=49;boxesLinked[40][7]=50;boxesLinked[40][8]=36;boxesLinked[40][9]=37;boxesLinked[40][10]=38;boxesLinked[40][11]=42;boxesLinked[40][12]=43;boxesLinked[40][13]=44;boxesLinked[40][14]=4;boxesLinked[40][15]=13;boxesLinked[40][16]=22;boxesLinked[40][17]=58;boxesLinked[40][18]=67;boxesLinked[40][19]=76;
    boxesLinked[41][0]=30;boxesLinked[41][1]=31;boxesLinked[41][2]=32;boxesLinked[41][3]=39;boxesLinked[41][4]=40;boxesLinked[41][5]=48;boxesLinked[41][6]=49;boxesLinked[41][7]=50;boxesLinked[41][8]=36;boxesLinked[41][9]=37;boxesLinked[41][10]=38;boxesLinked[41][11]=42;boxesLinked[41][12]=43;boxesLinked[41][13]=44;boxesLinked[41][14]=5;boxesLinked[41][15]=14;boxesLinked[41][16]=23;boxesLinked[41][17]=59;boxesLinked[41][18]=68;boxesLinked[41][19]=77;
    boxesLinked[42][0]=33;boxesLinked[42][1]=34;boxesLinked[42][2]=35;boxesLinked[42][3]=43;boxesLinked[42][4]=44;boxesLinked[42][5]=51;boxesLinked[42][6]=52;boxesLinked[42][7]=53;boxesLinked[42][8]=36;boxesLinked[42][9]=37;boxesLinked[42][10]=38;boxesLinked[42][11]=39;boxesLinked[42][12]=40;boxesLinked[42][13]=41;boxesLinked[42][14]=6;boxesLinked[42][15]=15;boxesLinked[42][16]=24;boxesLinked[42][17]=60;boxesLinked[42][18]=69;boxesLinked[42][19]=78;
    boxesLinked[43][0]=33;boxesLinked[43][1]=34;boxesLinked[43][2]=35;boxesLinked[43][3]=42;boxesLinked[43][4]=44;boxesLinked[43][5]=51;boxesLinked[43][6]=52;boxesLinked[43][7]=53;boxesLinked[43][8]=36;boxesLinked[43][9]=37;boxesLinked[43][10]=38;boxesLinked[43][11]=39;boxesLinked[43][12]=40;boxesLinked[43][13]=41;boxesLinked[43][14]=7;boxesLinked[43][15]=16;boxesLinked[43][16]=25;boxesLinked[43][17]=61;boxesLinked[43][18]=70;boxesLinked[43][19]=79;
    boxesLinked[44][0]=33;boxesLinked[44][1]=34;boxesLinked[44][2]=35;boxesLinked[44][3]=42;boxesLinked[44][4]=43;boxesLinked[44][5]=51;boxesLinked[44][6]=52;boxesLinked[44][7]=53;boxesLinked[44][8]=36;boxesLinked[44][9]=37;boxesLinked[44][10]=38;boxesLinked[44][11]=39;boxesLinked[44][12]=40;boxesLinked[44][13]=41;boxesLinked[44][14]=8;boxesLinked[44][15]=17;boxesLinked[44][16]=26;boxesLinked[44][17]=62;boxesLinked[44][18]=71;boxesLinked[44][19]=80;
    boxesLinked[45][0]=27;boxesLinked[45][1]=28;boxesLinked[45][2]=29;boxesLinked[45][3]=36;boxesLinked[45][4]=37;boxesLinked[45][5]=38;boxesLinked[45][6]=46;boxesLinked[45][7]=47;boxesLinked[45][8]=48;boxesLinked[45][9]=49;boxesLinked[45][10]=50;boxesLinked[45][11]=51;boxesLinked[45][12]=52;boxesLinked[45][13]=53;boxesLinked[45][14]=0;boxesLinked[45][15]=9;boxesLinked[45][16]=18;boxesLinked[45][17]=54;boxesLinked[45][18]=63;boxesLinked[45][19]=72;
    boxesLinked[46][0]=27;boxesLinked[46][1]=28;boxesLinked[46][2]=29;boxesLinked[46][3]=36;boxesLinked[46][4]=37;boxesLinked[46][5]=38;boxesLinked[46][6]=45;boxesLinked[46][7]=47;boxesLinked[46][8]=48;boxesLinked[46][9]=49;boxesLinked[46][10]=50;boxesLinked[46][11]=51;boxesLinked[46][12]=52;boxesLinked[46][13]=53;boxesLinked[46][14]=1;boxesLinked[46][15]=10;boxesLinked[46][16]=19;boxesLinked[46][17]=55;boxesLinked[46][18]=64;boxesLinked[46][19]=73;
    boxesLinked[47][0]=27;boxesLinked[47][1]=28;boxesLinked[47][2]=29;boxesLinked[47][3]=36;boxesLinked[47][4]=37;boxesLinked[47][5]=38;boxesLinked[47][6]=45;boxesLinked[47][7]=46;boxesLinked[47][8]=48;boxesLinked[47][9]=49;boxesLinked[47][10]=50;boxesLinked[47][11]=51;boxesLinked[47][12]=52;boxesLinked[47][13]=53;boxesLinked[47][14]=2;boxesLinked[47][15]=11;boxesLinked[47][16]=20;boxesLinked[47][17]=56;boxesLinked[47][18]=65;boxesLinked[47][19]=74;
    boxesLinked[48][0]=30;boxesLinked[48][1]=31;boxesLinked[48][2]=32;boxesLinked[48][3]=39;boxesLinked[48][4]=40;boxesLinked[48][5]=41;boxesLinked[48][6]=49;boxesLinked[48][7]=50;boxesLinked[48][8]=45;boxesLinked[48][9]=46;boxesLinked[48][10]=47;boxesLinked[48][11]=51;boxesLinked[48][12]=52;boxesLinked[48][13]=53;boxesLinked[48][14]=3;boxesLinked[48][15]=12;boxesLinked[48][16]=21;boxesLinked[48][17]=57;boxesLinked[48][18]=66;boxesLinked[48][19]=75;
    boxesLinked[49][0]=30;boxesLinked[49][1]=31;boxesLinked[49][2]=32;boxesLinked[49][3]=39;boxesLinked[49][4]=40;boxesLinked[49][5]=41;boxesLinked[49][6]=48;boxesLinked[49][7]=50;boxesLinked[49][8]=45;boxesLinked[49][9]=46;boxesLinked[49][10]=47;boxesLinked[49][11]=51;boxesLinked[49][12]=52;boxesLinked[49][13]=53;boxesLinked[49][14]=4;boxesLinked[49][15]=13;boxesLinked[49][16]=22;boxesLinked[49][17]=58;boxesLinked[49][18]=67;boxesLinked[49][19]=76;
    boxesLinked[50][0]=30;boxesLinked[50][1]=31;boxesLinked[50][2]=32;boxesLinked[50][3]=39;boxesLinked[50][4]=40;boxesLinked[50][5]=41;boxesLinked[50][6]=48;boxesLinked[50][7]=49;boxesLinked[50][8]=45;boxesLinked[50][9]=46;boxesLinked[50][10]=47;boxesLinked[50][11]=51;boxesLinked[50][12]=52;boxesLinked[50][13]=53;boxesLinked[50][14]=5;boxesLinked[50][15]=14;boxesLinked[50][16]=23;boxesLinked[50][17]=59;boxesLinked[50][18]=68;boxesLinked[50][19]=77;
    boxesLinked[51][0]=33;boxesLinked[51][1]=34;boxesLinked[51][2]=35;boxesLinked[51][3]=42;boxesLinked[51][4]=43;boxesLinked[51][5]=44;boxesLinked[51][6]=52;boxesLinked[51][7]=53;boxesLinked[51][8]=45;boxesLinked[51][9]=46;boxesLinked[51][10]=47;boxesLinked[51][11]=48;boxesLinked[51][12]=49;boxesLinked[51][13]=50;boxesLinked[51][14]=6;boxesLinked[51][15]=15;boxesLinked[51][16]=24;boxesLinked[51][17]=60;boxesLinked[51][18]=69;boxesLinked[51][19]=78;
    boxesLinked[52][0]=33;boxesLinked[52][1]=34;boxesLinked[52][2]=35;boxesLinked[52][3]=42;boxesLinked[52][4]=43;boxesLinked[52][5]=44;boxesLinked[52][6]=51;boxesLinked[52][7]=53;boxesLinked[52][8]=45;boxesLinked[52][9]=46;boxesLinked[52][10]=47;boxesLinked[52][11]=48;boxesLinked[52][12]=49;boxesLinked[52][13]=50;boxesLinked[52][14]=7;boxesLinked[52][15]=16;boxesLinked[52][16]=25;boxesLinked[52][17]=61;boxesLinked[52][18]=70;boxesLinked[52][19]=79;
    boxesLinked[53][0]=33;boxesLinked[53][1]=34;boxesLinked[53][2]=35;boxesLinked[53][3]=42;boxesLinked[53][4]=43;boxesLinked[53][5]=44;boxesLinked[53][6]=51;boxesLinked[53][7]=52;boxesLinked[53][8]=45;boxesLinked[53][9]=46;boxesLinked[53][10]=47;boxesLinked[53][11]=48;boxesLinked[53][12]=49;boxesLinked[53][13]=50;boxesLinked[53][14]=8;boxesLinked[53][15]=17;boxesLinked[53][16]=26;boxesLinked[53][17]=62;boxesLinked[53][18]=71;boxesLinked[53][19]=80;
    boxesLinked[54][0]=55;boxesLinked[54][1]=56;boxesLinked[54][2]=63;boxesLinked[54][3]=64;boxesLinked[54][4]=65;boxesLinked[54][5]=72;boxesLinked[54][6]=73;boxesLinked[54][7]=74;boxesLinked[54][8]=57;boxesLinked[54][9]=58;boxesLinked[54][10]=59;boxesLinked[54][11]=60;boxesLinked[54][12]=61;boxesLinked[54][13]=62;boxesLinked[54][14]=0;boxesLinked[54][15]=9;boxesLinked[54][16]=18;boxesLinked[54][17]=27;boxesLinked[54][18]=36;boxesLinked[54][19]=45;
    boxesLinked[55][0]=54;boxesLinked[55][1]=56;boxesLinked[55][2]=63;boxesLinked[55][3]=64;boxesLinked[55][4]=65;boxesLinked[55][5]=72;boxesLinked[55][6]=73;boxesLinked[55][7]=74;boxesLinked[55][8]=57;boxesLinked[55][9]=58;boxesLinked[55][10]=59;boxesLinked[55][11]=60;boxesLinked[55][12]=61;boxesLinked[55][13]=62;boxesLinked[55][14]=1;boxesLinked[55][15]=10;boxesLinked[55][16]=19;boxesLinked[55][17]=28;boxesLinked[55][18]=37;boxesLinked[55][19]=46;
    boxesLinked[56][0]=54;boxesLinked[56][1]=55;boxesLinked[56][2]=63;boxesLinked[56][3]=64;boxesLinked[56][4]=65;boxesLinked[56][5]=72;boxesLinked[56][6]=73;boxesLinked[56][7]=74;boxesLinked[56][8]=57;boxesLinked[56][9]=58;boxesLinked[56][10]=59;boxesLinked[56][11]=60;boxesLinked[56][12]=61;boxesLinked[56][13]=62;boxesLinked[56][14]=2;boxesLinked[56][15]=11;boxesLinked[56][16]=20;boxesLinked[56][17]=29;boxesLinked[56][18]=38;boxesLinked[56][19]=47;
    boxesLinked[57][0]=58;boxesLinked[57][1]=59;boxesLinked[57][2]=66;boxesLinked[57][3]=67;boxesLinked[57][4]=68;boxesLinked[57][5]=75;boxesLinked[57][6]=76;boxesLinked[57][7]=77;boxesLinked[57][8]=54;boxesLinked[57][9]=55;boxesLinked[57][10]=56;boxesLinked[57][11]=60;boxesLinked[57][12]=61;boxesLinked[57][13]=62;boxesLinked[57][14]=3;boxesLinked[57][15]=12;boxesLinked[57][16]=21;boxesLinked[57][17]=30;boxesLinked[57][18]=39;boxesLinked[57][19]=48;
    boxesLinked[58][0]=57;boxesLinked[58][1]=59;boxesLinked[58][2]=66;boxesLinked[58][3]=67;boxesLinked[58][4]=68;boxesLinked[58][5]=75;boxesLinked[58][6]=76;boxesLinked[58][7]=77;boxesLinked[58][8]=54;boxesLinked[58][9]=55;boxesLinked[58][10]=56;boxesLinked[58][11]=60;boxesLinked[58][12]=61;boxesLinked[58][13]=62;boxesLinked[58][14]=4;boxesLinked[58][15]=13;boxesLinked[58][16]=22;boxesLinked[58][17]=31;boxesLinked[58][18]=40;boxesLinked[58][19]=49;
    boxesLinked[59][0]=57;boxesLinked[59][1]=58;boxesLinked[59][2]=66;boxesLinked[59][3]=67;boxesLinked[59][4]=68;boxesLinked[59][5]=75;boxesLinked[59][6]=76;boxesLinked[59][7]=77;boxesLinked[59][8]=54;boxesLinked[59][9]=55;boxesLinked[59][10]=56;boxesLinked[59][11]=60;boxesLinked[59][12]=61;boxesLinked[59][13]=62;boxesLinked[59][14]=5;boxesLinked[59][15]=14;boxesLinked[59][16]=23;boxesLinked[59][17]=32;boxesLinked[59][18]=41;boxesLinked[59][19]=50;
    boxesLinked[60][0]=61;boxesLinked[60][1]=62;boxesLinked[60][2]=69;boxesLinked[60][3]=70;boxesLinked[60][4]=71;boxesLinked[60][5]=78;boxesLinked[60][6]=79;boxesLinked[60][7]=80;boxesLinked[60][8]=54;boxesLinked[60][9]=55;boxesLinked[60][10]=56;boxesLinked[60][11]=57;boxesLinked[60][12]=58;boxesLinked[60][13]=59;boxesLinked[60][14]=6;boxesLinked[60][15]=15;boxesLinked[60][16]=24;boxesLinked[60][17]=33;boxesLinked[60][18]=42;boxesLinked[60][19]=51;
    boxesLinked[61][0]=60;boxesLinked[61][1]=62;boxesLinked[61][2]=69;boxesLinked[61][3]=70;boxesLinked[61][4]=71;boxesLinked[61][5]=78;boxesLinked[61][6]=79;boxesLinked[61][7]=80;boxesLinked[61][8]=54;boxesLinked[61][9]=55;boxesLinked[61][10]=56;boxesLinked[61][11]=57;boxesLinked[61][12]=58;boxesLinked[61][13]=59;boxesLinked[61][14]=7;boxesLinked[61][15]=16;boxesLinked[61][16]=25;boxesLinked[61][17]=34;boxesLinked[61][18]=43;boxesLinked[61][19]=52;
    boxesLinked[62][0]=60;boxesLinked[62][1]=61;boxesLinked[62][2]=69;boxesLinked[62][3]=70;boxesLinked[62][4]=71;boxesLinked[62][5]=78;boxesLinked[62][6]=79;boxesLinked[62][7]=80;boxesLinked[62][8]=54;boxesLinked[62][9]=55;boxesLinked[62][10]=56;boxesLinked[62][11]=57;boxesLinked[62][12]=58;boxesLinked[62][13]=59;boxesLinked[62][14]=8;boxesLinked[62][15]=17;boxesLinked[62][16]=26;boxesLinked[62][17]=35;boxesLinked[62][18]=44;boxesLinked[62][19]=53;
    boxesLinked[63][0]=54;boxesLinked[63][1]=55;boxesLinked[63][2]=56;boxesLinked[63][3]=64;boxesLinked[63][4]=65;boxesLinked[63][5]=72;boxesLinked[63][6]=73;boxesLinked[63][7]=74;boxesLinked[63][8]=66;boxesLinked[63][9]=67;boxesLinked[63][10]=68;boxesLinked[63][11]=69;boxesLinked[63][12]=70;boxesLinked[63][13]=71;boxesLinked[63][14]=0;boxesLinked[63][15]=9;boxesLinked[63][16]=18;boxesLinked[63][17]=27;boxesLinked[63][18]=36;boxesLinked[63][19]=45;
    boxesLinked[64][0]=54;boxesLinked[64][1]=55;boxesLinked[64][2]=56;boxesLinked[64][3]=63;boxesLinked[64][4]=65;boxesLinked[64][5]=72;boxesLinked[64][6]=73;boxesLinked[64][7]=74;boxesLinked[64][8]=66;boxesLinked[64][9]=67;boxesLinked[64][10]=68;boxesLinked[64][11]=69;boxesLinked[64][12]=70;boxesLinked[64][13]=71;boxesLinked[64][14]=1;boxesLinked[64][15]=10;boxesLinked[64][16]=19;boxesLinked[64][17]=28;boxesLinked[64][18]=37;boxesLinked[64][19]=46;
    boxesLinked[65][0]=54;boxesLinked[65][1]=55;boxesLinked[65][2]=56;boxesLinked[65][3]=63;boxesLinked[65][4]=64;boxesLinked[65][5]=72;boxesLinked[65][6]=73;boxesLinked[65][7]=74;boxesLinked[65][8]=66;boxesLinked[65][9]=67;boxesLinked[65][10]=68;boxesLinked[65][11]=69;boxesLinked[65][12]=70;boxesLinked[65][13]=71;boxesLinked[65][14]=2;boxesLinked[65][15]=11;boxesLinked[65][16]=20;boxesLinked[65][17]=29;boxesLinked[65][18]=38;boxesLinked[65][19]=47;
    boxesLinked[66][0]=57;boxesLinked[66][1]=58;boxesLinked[66][2]=59;boxesLinked[66][3]=67;boxesLinked[66][4]=68;boxesLinked[66][5]=75;boxesLinked[66][6]=76;boxesLinked[66][7]=77;boxesLinked[66][8]=63;boxesLinked[66][9]=64;boxesLinked[66][10]=65;boxesLinked[66][11]=69;boxesLinked[66][12]=70;boxesLinked[66][13]=71;boxesLinked[66][14]=3;boxesLinked[66][15]=12;boxesLinked[66][16]=21;boxesLinked[66][17]=30;boxesLinked[66][18]=39;boxesLinked[66][19]=48;
    boxesLinked[67][0]=57;boxesLinked[67][1]=58;boxesLinked[67][2]=59;boxesLinked[67][3]=66;boxesLinked[67][4]=68;boxesLinked[67][5]=75;boxesLinked[67][6]=76;boxesLinked[67][7]=77;boxesLinked[67][8]=63;boxesLinked[67][9]=64;boxesLinked[67][10]=65;boxesLinked[67][11]=69;boxesLinked[67][12]=70;boxesLinked[67][13]=71;boxesLinked[67][14]=4;boxesLinked[67][15]=13;boxesLinked[67][16]=22;boxesLinked[67][17]=31;boxesLinked[67][18]=40;boxesLinked[67][19]=49;
    boxesLinked[68][0]=57;boxesLinked[68][1]=58;boxesLinked[68][2]=59;boxesLinked[68][3]=66;boxesLinked[68][4]=67;boxesLinked[68][5]=75;boxesLinked[68][6]=76;boxesLinked[68][7]=77;boxesLinked[68][8]=63;boxesLinked[68][9]=64;boxesLinked[68][10]=65;boxesLinked[68][11]=69;boxesLinked[68][12]=70;boxesLinked[68][13]=71;boxesLinked[68][14]=5;boxesLinked[68][15]=14;boxesLinked[68][16]=23;boxesLinked[68][17]=32;boxesLinked[68][18]=41;boxesLinked[68][19]=50;
    boxesLinked[69][0]=60;boxesLinked[69][1]=61;boxesLinked[69][2]=62;boxesLinked[69][3]=70;boxesLinked[69][4]=71;boxesLinked[69][5]=78;boxesLinked[69][6]=79;boxesLinked[69][7]=80;boxesLinked[69][8]=63;boxesLinked[69][9]=64;boxesLinked[69][10]=65;boxesLinked[69][11]=66;boxesLinked[69][12]=67;boxesLinked[69][13]=68;boxesLinked[69][14]=6;boxesLinked[69][15]=15;boxesLinked[69][16]=24;boxesLinked[69][17]=33;boxesLinked[69][18]=42;boxesLinked[69][19]=51;
    boxesLinked[70][0]=60;boxesLinked[70][1]=61;boxesLinked[70][2]=62;boxesLinked[70][3]=69;boxesLinked[70][4]=71;boxesLinked[70][5]=78;boxesLinked[70][6]=79;boxesLinked[70][7]=80;boxesLinked[70][8]=63;boxesLinked[70][9]=64;boxesLinked[70][10]=65;boxesLinked[70][11]=66;boxesLinked[70][12]=67;boxesLinked[70][13]=68;boxesLinked[70][14]=7;boxesLinked[70][15]=16;boxesLinked[70][16]=25;boxesLinked[70][17]=34;boxesLinked[70][18]=43;boxesLinked[70][19]=52;
    boxesLinked[71][0]=60;boxesLinked[71][1]=61;boxesLinked[71][2]=62;boxesLinked[71][3]=69;boxesLinked[71][4]=70;boxesLinked[71][5]=78;boxesLinked[71][6]=79;boxesLinked[71][7]=80;boxesLinked[71][8]=63;boxesLinked[71][9]=64;boxesLinked[71][10]=65;boxesLinked[71][11]=66;boxesLinked[71][12]=67;boxesLinked[71][13]=68;boxesLinked[71][14]=8;boxesLinked[71][15]=17;boxesLinked[71][16]=26;boxesLinked[71][17]=35;boxesLinked[71][18]=44;boxesLinked[71][19]=53;
    boxesLinked[72][0]=54;boxesLinked[72][1]=55;boxesLinked[72][2]=56;boxesLinked[72][3]=63;boxesLinked[72][4]=64;boxesLinked[72][5]=65;boxesLinked[72][6]=73;boxesLinked[72][7]=74;boxesLinked[72][8]=75;boxesLinked[72][9]=76;boxesLinked[72][10]=77;boxesLinked[72][11]=78;boxesLinked[72][12]=79;boxesLinked[72][13]=80;boxesLinked[72][14]=0;boxesLinked[72][15]=9;boxesLinked[72][16]=18;boxesLinked[72][17]=27;boxesLinked[72][18]=36;boxesLinked[72][19]=45;
    boxesLinked[73][0]=54;boxesLinked[73][1]=55;boxesLinked[73][2]=56;boxesLinked[73][3]=63;boxesLinked[73][4]=64;boxesLinked[73][5]=65;boxesLinked[73][6]=72;boxesLinked[73][7]=74;boxesLinked[73][8]=75;boxesLinked[73][9]=76;boxesLinked[73][10]=77;boxesLinked[73][11]=78;boxesLinked[73][12]=79;boxesLinked[73][13]=80;boxesLinked[73][14]=1;boxesLinked[73][15]=10;boxesLinked[73][16]=19;boxesLinked[73][17]=28;boxesLinked[73][18]=37;boxesLinked[73][19]=46;
    boxesLinked[74][0]=54;boxesLinked[74][1]=55;boxesLinked[74][2]=56;boxesLinked[74][3]=63;boxesLinked[74][4]=64;boxesLinked[74][5]=65;boxesLinked[74][6]=72;boxesLinked[74][7]=73;boxesLinked[74][8]=75;boxesLinked[74][9]=76;boxesLinked[74][10]=77;boxesLinked[74][11]=78;boxesLinked[74][12]=79;boxesLinked[74][13]=80;boxesLinked[74][14]=2;boxesLinked[74][15]=11;boxesLinked[74][16]=20;boxesLinked[74][17]=29;boxesLinked[74][18]=38;boxesLinked[74][19]=47;
    boxesLinked[75][0]=57;boxesLinked[75][1]=58;boxesLinked[75][2]=59;boxesLinked[75][3]=66;boxesLinked[75][4]=67;boxesLinked[75][5]=68;boxesLinked[75][6]=76;boxesLinked[75][7]=77;boxesLinked[75][8]=72;boxesLinked[75][9]=73;boxesLinked[75][10]=74;boxesLinked[75][11]=78;boxesLinked[75][12]=79;boxesLinked[75][13]=80;boxesLinked[75][14]=3;boxesLinked[75][15]=12;boxesLinked[75][16]=21;boxesLinked[75][17]=30;boxesLinked[75][18]=39;boxesLinked[75][19]=48;
    boxesLinked[76][0]=57;boxesLinked[76][1]=58;boxesLinked[76][2]=59;boxesLinked[76][3]=66;boxesLinked[76][4]=67;boxesLinked[76][5]=68;boxesLinked[76][6]=75;boxesLinked[76][7]=77;boxesLinked[76][8]=72;boxesLinked[76][9]=73;boxesLinked[76][10]=74;boxesLinked[76][11]=78;boxesLinked[76][12]=79;boxesLinked[76][13]=80;boxesLinked[76][14]=4;boxesLinked[76][15]=13;boxesLinked[76][16]=22;boxesLinked[76][17]=31;boxesLinked[76][18]=40;boxesLinked[76][19]=49;
    boxesLinked[77][0]=57;boxesLinked[77][1]=58;boxesLinked[77][2]=59;boxesLinked[77][3]=66;boxesLinked[77][4]=67;boxesLinked[77][5]=68;boxesLinked[77][6]=75;boxesLinked[77][7]=76;boxesLinked[77][8]=72;boxesLinked[77][9]=73;boxesLinked[77][10]=74;boxesLinked[77][11]=78;boxesLinked[77][12]=79;boxesLinked[77][13]=80;boxesLinked[77][14]=5;boxesLinked[77][15]=14;boxesLinked[77][16]=23;boxesLinked[77][17]=32;boxesLinked[77][18]=41;boxesLinked[77][19]=50;
    boxesLinked[78][0]=60;boxesLinked[78][1]=61;boxesLinked[78][2]=62;boxesLinked[78][3]=69;boxesLinked[78][4]=70;boxesLinked[78][5]=71;boxesLinked[78][6]=79;boxesLinked[78][7]=80;boxesLinked[78][8]=72;boxesLinked[78][9]=73;boxesLinked[78][10]=74;boxesLinked[78][11]=75;boxesLinked[78][12]=76;boxesLinked[78][13]=77;boxesLinked[78][14]=6;boxesLinked[78][15]=15;boxesLinked[78][16]=24;boxesLinked[78][17]=33;boxesLinked[78][18]=42;boxesLinked[78][19]=51;
    boxesLinked[79][0]=60;boxesLinked[79][1]=61;boxesLinked[79][2]=62;boxesLinked[79][3]=69;boxesLinked[79][4]=70;boxesLinked[79][5]=71;boxesLinked[79][6]=78;boxesLinked[79][7]=80;boxesLinked[79][8]=72;boxesLinked[79][9]=73;boxesLinked[79][10]=74;boxesLinked[79][11]=75;boxesLinked[79][12]=76;boxesLinked[79][13]=77;boxesLinked[79][14]=7;boxesLinked[79][15]=16;boxesLinked[79][16]=25;boxesLinked[79][17]=34;boxesLinked[79][18]=43;boxesLinked[79][19]=52;
    boxesLinked[80][0]=60;boxesLinked[80][1]=61;boxesLinked[80][2]=62;boxesLinked[80][3]=69;boxesLinked[80][4]=70;boxesLinked[80][5]=71;boxesLinked[80][6]=78;boxesLinked[80][7]=79;boxesLinked[80][8]=72;boxesLinked[80][9]=73;boxesLinked[80][10]=74;boxesLinked[80][11]=75;boxesLinked[80][12]=76;boxesLinked[80][13]=77;boxesLinked[80][14]=8;boxesLinked[80][15]=17;boxesLinked[80][16]=26;boxesLinked[80][17]=35;boxesLinked[80][18]=44;boxesLinked[80][19]=53;

    listBox[0]=ui->comboBox00; listBox[1]=ui->comboBox01; listBox[2]=ui->comboBox02;
    listBox[3]=ui->comboBox03; listBox[4]=ui->comboBox04; listBox[5]=ui->comboBox05;
    listBox[6]=ui->comboBox06; listBox[7]=ui->comboBox07; listBox[8]=ui->comboBox08;
    listBox[9]=ui->comboBox09; listBox[10]=ui->comboBox10;listBox[11]=ui->comboBox11;
    listBox[12]=ui->comboBox12;listBox[13]=ui->comboBox13;listBox[14]=ui->comboBox14;
    listBox[15]=ui->comboBox15;listBox[16]=ui->comboBox16;listBox[17]=ui->comboBox17;
    listBox[18]=ui->comboBox18;listBox[19]=ui->comboBox19;listBox[20]=ui->comboBox20;
    listBox[21]=ui->comboBox21;listBox[22]=ui->comboBox22;listBox[23]=ui->comboBox23;
    listBox[24]=ui->comboBox24;listBox[25]=ui->comboBox25;listBox[26]=ui->comboBox26;
    listBox[27]=ui->comboBox27;listBox[28]=ui->comboBox28;listBox[29]=ui->comboBox29;
    listBox[30]=ui->comboBox30;listBox[31]=ui->comboBox31;listBox[32]=ui->comboBox32;
    listBox[33]=ui->comboBox33;listBox[34]=ui->comboBox34;listBox[35]=ui->comboBox35;
    listBox[36]=ui->comboBox36;listBox[37]=ui->comboBox37;listBox[38]=ui->comboBox38;
    listBox[39]=ui->comboBox39;listBox[40]=ui->comboBox40;listBox[41]=ui->comboBox41;
    listBox[42]=ui->comboBox42;listBox[43]=ui->comboBox43;listBox[44]=ui->comboBox44;
    listBox[45]=ui->comboBox45;listBox[46]=ui->comboBox46;listBox[47]=ui->comboBox47;
    listBox[48]=ui->comboBox48;listBox[49]=ui->comboBox49;listBox[50]=ui->comboBox50;
    listBox[51]=ui->comboBox51;listBox[52]=ui->comboBox52;listBox[53]=ui->comboBox53;
    listBox[54]=ui->comboBox54;listBox[55]=ui->comboBox55;listBox[56]=ui->comboBox56;
    listBox[57]=ui->comboBox57;listBox[58]=ui->comboBox58;listBox[59]=ui->comboBox59;
    listBox[60]=ui->comboBox60;listBox[61]=ui->comboBox61;listBox[62]=ui->comboBox62;
    listBox[63]=ui->comboBox63;listBox[64]=ui->comboBox64;listBox[65]=ui->comboBox65;
    listBox[66]=ui->comboBox66;listBox[67]=ui->comboBox67;listBox[68]=ui->comboBox68;
    listBox[69]=ui->comboBox69;listBox[70]=ui->comboBox70;listBox[71]=ui->comboBox71;
    listBox[72]=ui->comboBox72;listBox[73]=ui->comboBox73;listBox[74]=ui->comboBox74;
    listBox[75]=ui->comboBox75;listBox[76]=ui->comboBox76;listBox[77]=ui->comboBox77;
    listBox[78]=ui->comboBox78;listBox[79]=ui->comboBox79;listBox[80]=ui->comboBox80;
}
