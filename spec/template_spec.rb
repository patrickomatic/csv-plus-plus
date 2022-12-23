require 'template'

describe GSPush::Template do
  let(:input) { "foo,bar,baz" }
  let(:template) { GSPush::Template.new(input) }

  describe "process!" do
    before(:each) { template.process! }

    it "creates rows" do
      expect(template.rows.length).to eq(1)
    end

    context "with cell modifiers" do
      let(:input) { "foo,<[align=right/format=bold]>bar,baz" }

      it "creates cells with the modifiers" do
        pp template.rows[0].cells
        expect(template.rows[0].cells[1].align).to eq('right')
        expect(template.rows[0].cells[1].formats).to eq(['bold'])
      end
    end

    context "with cell modifiers with multiple values" do
      let(:input) { "foo,<[align=right/format=bold italic]>bar,baz" }

      it "creates cells with the modifiers" do
        pp template.rows[0].cells
        expect(template.rows[0].cells[1].formats).to eq(['bold', 'italic'])
      end
    end


    context "with row modifiers" do
      let(:input) { "<![align=center/format=bold]>foo,bar,baz" }

      it "creates rows with the modifiers" do
        expect(template.rows[0].align).to eq('center')
        expect(template.rows[0].format).to eq(['bold'])
      end
    end
  end
end
