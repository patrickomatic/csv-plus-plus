require 'modifier'

describe GSPush::Modifier do
  describe ".get_modifier_from_value" do
    subject(:modifier) { GSPush::Modifier.get_modifier_from_value(cell_value) }
    let(:cell_value) { "<[format=bold]>bar" }

    describe "format=" do
      it "handles a single format" do
        is_expected.to be_bold
      end

      context "with multiple formats" do
        let(:cell_value) { "<[format=bold italic]>bar" }

        it { should be_bold }
        it { should be_italic }
      end

      context "with invalid formats" do
        let(:cell_value) { "<[format=foo]>bar" }

        it "throws a syntax error" do
          expect { subject }.to raise_error(GSPush::Modifier::SyntaxError)
        end
      end
    end

    describe "align=" do
      let(:cell_value) { "<[align=center]>foo" }

      it "handles a single align" do
        expect(subject.align).to eq("center")
      end

      context "with invalid align" do
        let(:cell_value) { "<[align=foo]>foo" }

        it "throws a syntax error" do
          expect { subject }.to raise_error(GSPush::Modifier::SyntaxError)
        end
      end
    end
  end 
end
