
namespace SpeakerMachine
{
    partial class MainForm
    {
        /// <summary>
        ///  Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        ///  Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        ///  Required method for Designer support - do not modify
        ///  the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            tableLayoutPanel1 = new TableLayoutPanel();
            TitleTextBox = new RichTextBox();
            NameTextBox = new RichTextBox();
            DescriptionTextBox = new RichTextBox();
            SubmitButton = new Button();
            tableLayoutPanel1.SuspendLayout();
            SuspendLayout();
            // 
            // tableLayoutPanel1
            // 
            tableLayoutPanel1.ColumnCount = 2;
            tableLayoutPanel1.ColumnStyles.Add(new ColumnStyle(SizeType.Percent, 76.92308F));
            tableLayoutPanel1.ColumnStyles.Add(new ColumnStyle(SizeType.Percent, 23.0769234F));
            tableLayoutPanel1.Controls.Add(TitleTextBox, 0, 0);
            tableLayoutPanel1.Controls.Add(NameTextBox, 1, 0);
            tableLayoutPanel1.Controls.Add(DescriptionTextBox, 0, 1);
            tableLayoutPanel1.Controls.Add(SubmitButton, 1, 2);
            tableLayoutPanel1.Dock = DockStyle.Fill;
            tableLayoutPanel1.Location = new Point(0, 0);
            tableLayoutPanel1.Name = "tableLayoutPanel1";
            tableLayoutPanel1.RowCount = 3;
            tableLayoutPanel1.RowStyles.Add(new RowStyle(SizeType.Absolute, 40F));
            tableLayoutPanel1.RowStyles.Add(new RowStyle(SizeType.Percent, 100F));
            tableLayoutPanel1.RowStyles.Add(new RowStyle(SizeType.Absolute, 40F));
            tableLayoutPanel1.Size = new Size(509, 450);
            tableLayoutPanel1.TabIndex = 0;
            // 
            // TitleTextBox
            // 
            TitleTextBox.Dock = DockStyle.Fill;
            TitleTextBox.Location = new Point(3, 3);
            TitleTextBox.Multiline = false;
            TitleTextBox.Name = "TitleTextBox";
            TitleTextBox.Size = new Size(385, 34);
            TitleTextBox.TabIndex = 0;
            TitleTextBox.Text = "";
            // 
            // NameTextBox
            // 
            NameTextBox.Dock = DockStyle.Fill;
            NameTextBox.Location = new Point(394, 3);
            NameTextBox.Name = "NameTextBox";
            NameTextBox.Size = new Size(112, 34);
            NameTextBox.TabIndex = 1;
            NameTextBox.Text = "";
            // 
            // DescriptionTextBox
            // 
            tableLayoutPanel1.SetColumnSpan(DescriptionTextBox, 2);
            DescriptionTextBox.Dock = DockStyle.Fill;
            DescriptionTextBox.Location = new Point(3, 43);
            DescriptionTextBox.Name = "DescriptionTextBox";
            DescriptionTextBox.Size = new Size(503, 364);
            DescriptionTextBox.TabIndex = 2;
            DescriptionTextBox.Text = "";
            // 
            // SubmitButton
            // 
            SubmitButton.Dock = DockStyle.Fill;
            SubmitButton.Location = new Point(394, 413);
            SubmitButton.Name = "SubmitButton";
            SubmitButton.Size = new Size(112, 34);
            SubmitButton.TabIndex = 3;
            SubmitButton.Text = "Submit";
            SubmitButton.UseVisualStyleBackColor = true;
            SubmitButton.Click += SubmitButton_Click;
            // 
            // MainForm
            // 
            AutoScaleDimensions = new SizeF(9F, 20F);
            AutoScaleMode = AutoScaleMode.Font;
            ClientSize = new Size(509, 450);
            Controls.Add(tableLayoutPanel1);
            Name = "MainForm";
            Text = "Speaker Machine";
            tableLayoutPanel1.ResumeLayout(false);
            ResumeLayout(false);
        }

        #endregion

        private TableLayoutPanel tableLayoutPanel1;
        private RichTextBox TitleTextBox;
        private RichTextBox NameTextBox;
        private RichTextBox DescriptionTextBox;
        private Button SubmitButton;
    }
}
