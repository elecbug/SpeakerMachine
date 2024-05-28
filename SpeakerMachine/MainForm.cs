using Packet;
using System.Text;
using System.Text.Json;

namespace SpeakerMachine
{
    public partial class MainForm : Form
    {
        public MainForm()
        {
            InitializeComponent();
        }

        private void SubmitButton_Click(object sender, EventArgs e)
        {
            using (StreamReader reader = new StreamReader("config"))
            {
                string uri = reader.ReadToEnd();

                HttpClient client = new HttpClient();
                client.BaseAddress = new Uri(uri);

                Request request = new Request()
                {
                    Title = TitleTextBox.Text,
                    Description = DescriptionTextBox.Text,
                    Name = NameTextBox.Text,
                };

                string json = JsonSerializer.Serialize(request);

                HttpResponseMessage message 
                    = client.Send(new HttpRequestMessage(HttpMethod.Get, json));

                Stream stream = message.Content.ReadAsStream();
                byte[] bytes = new byte[stream.Length];

                stream.Read(bytes);

                MessageBox.Show(Encoding.UTF8.GetString(bytes));
            }
        }
    }
}
